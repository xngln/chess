use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
  Model{
    form: Form {
      username: String::new(),
      password: String::new(),
      
      errors: FormErrors::default(),
    }
  }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
  form: Form,
}

struct Form {
  username: String,
  password: String,

  errors: FormErrors,
}

#[derive(Default)]
struct FormErrors {
  username: Option<String>,
  password: Option<String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
  LoginComplete(Option<FetchError>),
  ClearErrors,
  UsernameEntered(String),
  PasswordEntered(String),
  
  Login,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
  match msg {
    Msg::LoginComplete(None) => {},
    Msg::LoginComplete(Some(fetch_error)) => {},
    Msg::ClearErrors => {},
    
    Msg::UsernameEntered(username) => {},
    Msg::PasswordEntered(password) => {},

    Msg::Login => {},
  }
}

// ------ ------
//     View
// ------ -------

pub fn view(model: &Model) -> Node<Msg> {
  div!["Login view"]
}