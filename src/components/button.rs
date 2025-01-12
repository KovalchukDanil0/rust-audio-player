use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, rsx, Attribute, Element, GlobalSignal, Props, Readable,
};

#[derive(Props, PartialEq, Clone)]
pub struct Props {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    extra_data: String,

    extra_data2: String,
}

#[component]
pub fn Button(props: Props) -> Element {
    rsx! {
        button {
            class: "float-left w-10 h-10 p-0 bg-transparent border-0 outline-none cursor-pointer",
            id: "play-icon",
            "{props.extra_data}"
        }
    }
}
