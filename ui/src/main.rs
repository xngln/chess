use seed::{prelude::*, *};

mod page;

const LOGIN: &str = "login";
const REGISTER: &str = "register";
const HOME: &str = "home";
const GAME: &str = "game";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::Landing,
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
    UrlChanged(subs::UrlChanged)
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>){
    match msg{
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("url changed", url)
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div!["Root view"]
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}