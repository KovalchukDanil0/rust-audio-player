use dioxus::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    let data = save_dog(event.value()).await.unwrap();
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
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));

    Ok("".to_string())
}
