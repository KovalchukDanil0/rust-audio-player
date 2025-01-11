use crate::components::Button;
use dioxus::prelude::{
    asset, component, dioxus_core, dioxus_elements, fc_to_builder, manganis, rsx, spawn,
    use_signal, Asset, Element, GlobalSignal, Props, Readable, Signal, Writable,
};

const ICON: Asset = asset!("/assets/icon.png");
const AUDIO_PLAY_ICON: Asset = asset!("/assets/icons/play.svg");
const AUDIO_PAUSE_ICON: Asset = asset!("/assets/icons/pause.svg");

#[derive(PartialEq, Props, Clone)]
pub struct Props {
    signal: Signal<bool>,
}

#[component]
pub fn AudioPlayer(props: Props) -> Element {
    let mut play_audio: Signal<bool> = use_signal(|| false);
    let play_audio_value: bool = *play_audio.read();

    let mut toggle_signal: Signal<bool> = props.signal;
    let toggle_signal_value: bool = *toggle_signal.read();

    rsx! {
        div { class: "flex flex-row justify-around h-full",

            div { class: "flex flex-row items-center justify-center h-full gap-3",

                img { class: "w-32 h-32", src: ICON }

                div { class: "flex flex-col gap-1",

                    p { "Song Name" }

                    p { "Album Name" }

                    button {
                        style: "padding: 10px 20px; background-color: #2ecc71; color: white; border: none; border-radius: 5px; cursor: pointer;",
                        onclick: move |_| toggle_signal.set(!toggle_signal_value),
                        "Toggle Value from Child"
                    }
                }
            }

            div { class: "flex flex-col items-center justify-center w-1/2 gap-3",

                audio {
                    muted: false,
                    src: "file:///home/gomofob/Music/ELUVEITIE%20-%20A%20Rose%20For%20Epona%20(OFFICIAL%20MUSIC%20VIDEO).mp3",
                    preload: "metadata",
                }

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

                Button { text: "Ggg" }

                button { class: "w-8 h-8", "Audio Indicator" }

                input { max: "100", r#type: "range" }
            }
        }
    }
}
