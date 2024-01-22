#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PropsType {
    pub label: String,
}

#[component]
pub fn Button(cx: Scope, props: PropsType) -> Element {
    let PropsType { label } = props;

    render! {
        button {
            class: "px-16 py-4 rounded-lg text-xl text-white bg-blue-500 hover:bg-blue-800",
            "{label}"
        }
    }
}
