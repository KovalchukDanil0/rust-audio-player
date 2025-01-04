use dioxus::prelude::{
    component, dioxus_core, dioxus_elements, rsx, server, server_fn, use_signal, Element,
    GlobalSignal, IntoDynNode, Readable, ServerFnError, Signal, Writable,
};

#[component]
pub fn Home() -> Element {
    let mut response: Signal<String> = use_signal(|| String::new());

    rsx! {
        div { "Track" }
        input {
            placeholder: "Type here to echo...",
            oninput: move |event| async move {
                let data = echo_server(event.value()).await.unwrap();
                response.set(data);
            },
        }
        if !response().is_empty() {
            p {
                "Server echoed: "
                i { "{response}" }
            }
        }
    }
}

#[server]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    use directories::UserDirs;
    use std::fs::read_dir;
    use std::path::PathBuf;

    let user_dirs: UserDirs = UserDirs::new().unwrap();
    let audio_dir: &str = user_dirs.audio_dir().unwrap().to_str().unwrap();

    let paths: Vec<PathBuf> = read_dir(audio_dir)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == "gif") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for path in paths {
        println!("Name: {}", path.to_str().unwrap())
    }

    Ok(input)
}
