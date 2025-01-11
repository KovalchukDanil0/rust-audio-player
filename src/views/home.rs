use crate::components::{AudioPlayer, Overview};
use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, rsx, server, server_fn, use_signal,
    Element, GlobalSignal, Readable, ServerFnError, Signal,
};
use gloo_file::{Blob, ObjectUrl};

#[component]
pub fn Home() -> Element {
    let gg: Signal<bool> = use_signal(|| true);
    let gg_value: bool = *gg.read();

    /*     let blob: Blob = Blob::new("hello world");
    let object_url: ObjectUrl = ObjectUrl::from(blob);

    println!("{}", object_url.to_string()); */

    rsx! {
        div { class: "h-screen",
            div { class: "flex flex-row h-4/5",
                div { class: "w-1/4 bg-red-500" }
                div { class: "w-full overflow-scroll bg-green-500", Overview {} }
            }
            div { class: "h-1/5 bg-blue-500",
                AudioPlayer { signal: gg }
                p { "{gg_value}" }
            }
        }
    }
}

#[server]
async fn echo_server() -> Result<Vec<[String; 2]>, ServerFnError> {
    use directories::UserDirs;
    use lofty::{file::TaggedFileExt, prelude::ItemKey, probe::Probe, tag::Tag};
    use std::path::Path;
    use walkdir::WalkDir;

    let user_dirs: UserDirs = UserDirs::new().unwrap();
    let audio_dir: &str = user_dirs.audio_dir().unwrap().to_str().unwrap();

    let files: Vec<[String; 2]> = WalkDir::new(audio_dir)
        .into_iter()
        .filter_map(|res| res.ok())
        .filter_map(|dir| {
            if dir.file_type().is_file() {
                let path: &Path = dir.path();

                let file_name: &str = path.file_stem().unwrap().to_str().unwrap();
                let full_path: String = path.to_string_lossy().into_owned();

                let tag: Tag = Probe::open(full_path.clone())
                    .unwrap()
                    .read()
                    .unwrap()
                    .primary_tag()
                    .unwrap()
                    .to_owned();

                let title: String = tag
                    .get_string(&ItemKey::TrackTitle)
                    .unwrap_or(file_name)
                    .to_string();
                let album: String = tag
                    .get_string(&ItemKey::AlbumTitle)
                    .unwrap_or("")
                    .to_string();

                println!("Title: {}, Album {}", title, album);

                Some([title, album])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(files)
}
