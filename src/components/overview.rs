use crate::utils::list_audio_files;
use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, rsx, spawn, use_effect, use_signal, Element,
    GlobalSignal, IntoDynNode, Props, Readable, Signal, Writable,
};

#[derive(PartialEq, Props, Clone)]
pub struct Props {
    audio_data: Signal<[String; 3]>,
}

#[component]
pub fn Overview(props: Props) -> Element {
    let mut audio_data: Signal<Vec<[String; 3]>> = use_signal(|| Vec::new());
    let mut current_audio_data: Signal<[String; 3]> = props.audio_data;

    use_effect(move || {
        spawn(async move {
            let data: Vec<[String; 3]> = list_audio_files().unwrap();
            audio_data.set(data);
        });
    });

    let mut play_music_on_click = move |audio_data: [String; 3]| {
        current_audio_data.set(audio_data);
    };

    rsx! {
        div { class: "flex flex-col gap-3",
            {
                audio_data
                    .read()
                    .iter()
                    .map(|[title, album, full_path]| {
                        let title = title.clone();
                        let album = album.clone();
                        let full_path = full_path.clone();
                        rsx! {
                            div { key: title, class: "flex flex-col gap-1",

                                p { "{title}" }
                                p { "{album}" }

                                button { onclick: move |_| play_music_on_click([title.clone(), album.clone(), full_path.clone()]),
                                    "Play Music"
                                }
                            }
                        }
                    })
            }
        }
    }
}
