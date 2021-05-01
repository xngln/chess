use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        form: Form {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),

            errors: FormErrors::default(),
        },
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
    confirm_password: String,

    errors: FormErrors,
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
    RegisterComplete(Option<FetchError>),
    ClearErrors,
    UsernameEntered(String),
    PasswordEntered(String),
    ConfirmPasswordEntered(String),

    Register,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::RegisterComplete(None) => {}
        Msg::RegisterComplete(Some(fetch_error)) => {}
        Msg::ClearErrors => {}

        Msg::UsernameEntered(username) => {}
        Msg::PasswordEntered(password) => {}
        Msg::ConfirmPasswordEntered(confirm_password) => {}

        Msg::Register => {}
    }
}

// ------ ------
//     View
// ------ -------

pub fn view(model: &Model) -> Node<Msg> {
    div!["Register view"]
}
