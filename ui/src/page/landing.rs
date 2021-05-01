use seed::{prelude::*, *};

// ------ ------
//     View
// ------ -------

pub fn view<Ms>() -> Node<Ms> {
    view_sidebar()
}

fn view_sidebar<Ms>() -> Node<Ms> {
    div![C!["grid grid-cols-2"], div!["1"], div!["2"],]
}
