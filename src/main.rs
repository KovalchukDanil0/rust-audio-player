mod components;
mod views;

use components::Navbar;
use dioxus::{
    document, launch,
    prelude::{
        asset, component, dioxus_core, dioxus_router, fc_to_builder, rsx, Asset, Element,
        GlobalSignal, Readable, Router, ToRouteSegments, VNode,
    },
};
use dioxus_router::prelude::Routable;
use manganis;
use views::{Blog, Home, NotFound};

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
