use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        game_state: String::new(),
        player: User {
            username: String::new(),
        },
        opponent: User {
            username: String::new(),
        },
    }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
    game_state: String,
    player: User,
    opponent: User,
}

struct User {
    username: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    MakeMove,
    InvalidMove,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        MakeMove => {}
        InvalidMove => {}
    }
}

// ------ ------
//     View
// ------ -------

pub fn view(model: &Model) -> Node<Msg> {
    div!["Game view"]
}
