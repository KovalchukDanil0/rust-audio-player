use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, rsx, server, server_fn, spawn, use_effect, use_signal,
    Element, GlobalSignal, IntoDynNode, Readable, ReadableVecExt, ServerFnError, Signal, Writable,
};

#[component]
pub fn Overview() -> Element {
    let mut response: Signal<Vec<[String; 2]>> = use_signal(|| Vec::new());

    use_effect(move || {
        spawn(async move {
            let data: Vec<[String; 2]> = echo_server().await.unwrap();
            response.set(data);
        });
    });

    rsx! {
        {response.iter().map(|items| rsx! {
            div {
                p { "{items[0]}" }
                if !items[1].is_empty() {
                    p { "{items[1]}" }
                }
            }
        })}
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
