use crate::assets;
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ -------

pub fn view<Ms>() -> Node<Ms> {
    div![C!["grid grid-cols-4"], view_sidebar(), view_main_panel(),]
}

fn view_sidebar<Ms>() -> Node<Ms> {
    div![
        img![
            C!["max-w-xs"],
            attrs! {
                At::Src => assets::url(assets::Image::Logo),
            }
        ],
        div!["michess",],
        div!["login",],
        div!["sign up"],
        div!["play as guest"],
    ]
}

fn view_main_panel<Ms>() -> Node<Ms> {
    div![
        C!["col-span-3"],
        img![attrs! {
            At::Src => assets::url(assets::Image::Board),
        }]
    ]
}
