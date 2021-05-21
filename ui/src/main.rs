use seed::{prelude::*, *};

mod assets;
mod page;

const LOGIN: &str = "login";
const REGISTER: &str = "register";
const HOME: &str = "home";
const GAME: &str = "game";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

struct User {
    username: String,
}

enum Page {
    Landing,
    Login(page::login::Model),
    Register(page::register::Model),
    Home(page::home::Model),
    Game(page::game::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Landing,
            [LOGIN] => Self::Login(page::login::init(url, &mut orders.proxy(Msg::Login))),
            [REGISTER] => {
                Self::Register(page::register::init(url, &mut orders.proxy(Msg::Register)))
            }
            [HOME] => Self::Home(page::home::init(url, &mut orders.proxy(Msg::Home))),
            [GAME] => Self::Game(page::game::init(url, &mut orders.proxy(Msg::Game))),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn landing(self) -> Url {
        self.base_url()
    }
    fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN)
    }
    fn register(self) -> Url {
        self.base_url().add_path_part(REGISTER)
    }
    fn home(self) -> Url {
        self.base_url().add_path_part(HOME)
    }
    fn game(self) -> Url {
        self.base_url().add_path_part(GAME)
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),

    Home(page::home::Msg),
    Login(page::login::Msg),
    Register(page::register::Msg),
    Game(page::game::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("url changed", url)
        }

        // ------ pages -------
        Msg::Login(msg) => {
            log!("got login msg: ", msg);
            if let Page::Login(model) = &mut model.page {
                page::login::update(msg, model, &mut orders.proxy(Msg::Login))
            }
        }
        Msg::Register(msg) => {
            log!("got register msg: ", msg);
            if let Page::Register(model) = &mut model.page {
                page::register::update(msg, model, &mut orders.proxy(Msg::Register))
            }
        }
        Msg::Home(msg) => {
            log!("got home msg: ", msg);
            if let Page::Home(model) = &mut model.page {
                page::home::update(msg, model, &mut orders.proxy(Msg::Home))
            }
        }
        Msg::Game(msg) => {
            log!("got game msg: ", msg);
            if let Page::Game(model) = &mut model.page {
                page::game::update(msg, model, &mut orders.proxy(Msg::Game))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    view_content(&model.page)
}

fn view_content(page: &Page) -> Node<Msg> {
    div![
        C!["container"],
        match page {
            Page::Landing => page::landing::view(),
            Page::Login(model) => page::login::view(model).map_msg(Msg::Login),
            Page::Register(model) => page::register::view(model).map_msg(Msg::Register),
            Page::Home(model) => page::home::view(model).map_msg(Msg::Home),
            Page::Game(model) => page::game::view(model).map_msg(Msg::Game),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}
