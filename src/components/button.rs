use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, rsx, Element, GlobalSignal, Props, Readable,
};

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    text: String,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "float-left w-10 h-10 p-0 bg-transparent border-0 outline-none cursor-pointer",
            id: "play-icon",
            "{props.text}"
        }
    }
}
