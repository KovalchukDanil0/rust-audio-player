use crate::components::{AudioPlayer, Overview};
use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, rsx, use_signal, Element, GlobalSignal,
    Readable, Signal,
};

#[component]
pub fn Home() -> Element {
    let audio_data: Signal<[String; 3]> = use_signal(|| {
        [
            "Song Title".to_string(),
            "Album Title".to_string(),
            String::new(),
        ]
    });

    rsx! {
        div { class: "h-screen",
            div { class: "flex flex-row h-4/5",
                div { class: "w-1/4 bg-red-500" }
                div { class: "w-full overflow-scroll bg-green-500 p-3",
                    Overview { audio_data }
                }
            }
            div { class: "h-1/5 bg-blue-500",
                AudioPlayer { audio_data }
            }
        }
    }
}
