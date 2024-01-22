#![allow(non_snake_case)]
pub mod components;

use components::button::{Button, PropsType as ButtonProps};
use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    render! {
        div {
            Button {
                props: ButtonProps {
                     label: String::from("Increment")
                 }
            }
            "Hello, World!"
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}
