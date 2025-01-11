mod components;
mod views;

use dioxus::prelude::{
    asset, component, dioxus_core, dioxus_router, document::Link, fc_to_builder, launch, manganis,
    rsx, Asset, Element, GlobalSignal, Readable, Router, ToRouteSegments, VNode,
};
use dioxus_router::prelude::Routable;
use views::{Home, NotFound};

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styles/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        Link { rel: "icon", href: FAVICON }
        Link { rel: "stylesheet", href: MAIN_CSS }
        Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
