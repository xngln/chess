use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
  Model{
    stats: Stats {
      wins: 0,
      losses: 0,
      draws: 0,
    },
    user: User {
      username: String::new(),
    },
  }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
  stats: Stats,
  user: User,
}

struct Stats {
  wins: u32,
  losses: u32,
  draws: u32,
}

struct User {
  username: String,
}

#[derive(Default)]
struct FormErrors {
  username: Option<String>,
  password: Option<String>,
  confirm_password: Option<String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
  SearchForOpponent(Option<FetchError>),
  ChallengeOpponent(Option<FetchError>,)
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::SearchForOpponent(None) => {},
    Msg::SearchForOpponent(Some(fetch_error)) => {},
    Msg::ChallengeOpponent(None) => {},
    Msg::ChallengeOpponent(Some(fetch_error)) => {},
  }
}

// ------ ------
//     View
// ------ -------

pub fn view(model: &Model) -> Node<Msg> {
  div!["Home view"]
}