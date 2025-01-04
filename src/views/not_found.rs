use crate::Route;
use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, rsx, Element, GlobalSignal, Link,
    Props, Readable,
};
use dioxus_class::prelude::{class, Class};

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: class!("flex" "flex-col" "items-center" "justify-center" "gap-3" "h-screen"),
            h1 { "Page Not Found - 404" }
            Link { to: Route::Home {}, "Return to homepage" }
        }
    }
}
