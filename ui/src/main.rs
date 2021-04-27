use seed::{prelude::*, *};

mod page;

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
    Landing(page::landing::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    Home(page::home::Model),
    Game(page::game::Model),
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