use crate::{components::Button, utils::play_music};
use dioxus::prelude::{
    asset, component, dioxus_core, dioxus_elements, fc_to_builder, manganis, rsx, use_signal,
    Asset, Element, GlobalAttributesExtension, GlobalSignal, Props, Readable, Signal, Writable,
};

const ICON: Asset = asset!("/assets/icon.png");
const AUDIO_PLAY_ICON: Asset = asset!("/assets/icons/play.svg");
const AUDIO_PAUSE_ICON: Asset = asset!("/assets/icons/pause.svg");

#[derive(PartialEq, Props, Clone)]
pub struct Props {
    audio_data: Signal<[String; 3]>,
}

#[component]
pub fn AudioPlayer(props: Props) -> Element {
    let mut play_audio: Signal<bool> = use_signal(|| false);
    let play_audio_value: bool = *play_audio.read();

    let audio_data: Signal<[String; 3]> = props.audio_data;
    let audio_data_value = audio_data.read();

    if !audio_data_value[2].is_empty() {
        play_music(audio_data_value[2].as_str());
    }

    rsx! {
        div { class: "flex flex-row justify-around h-full",

            div { class: "flex flex-row items-center justify-center h-full gap-3",

                img { class: "w-32 h-32", src: ICON }

                div { class: "flex flex-col gap-1",

                    p { "{audio_data_value[0]}" }

                    p { "{audio_data_value[1]}" }
                }
            }

            div { class: "flex flex-col items-center justify-center w-1/2 gap-3",

                button {
                    class: "float-left w-10 h-10 p-0 bg-transparent border-0 outline-none cursor-pointer",
                    id: "play-icon",

                    onclick: move |_| play_audio.set(!play_audio_value),

                    img { src: if play_audio_value { AUDIO_PAUSE_ICON } else { AUDIO_PLAY_ICON } }
                }

                div { class: "flex flex-row gap-3",

                    span { "Current Time" }

                    input { r#type: "range" }

                    output { "Duration Formatted" }
                }
            }

            div { class: "flex flex-row items-center justify-center gap-3",

                Button { class: "fgfg", extra_data: "rte", extra_data2: "fh" }

                button { class: "w-8 h-8", "Audio Indicator" }

                input { max: "100", r#type: "range" }
            }
        }
    }
}
