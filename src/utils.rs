use dioxus::prelude::ServerFnError;
use directories::UserDirs;
use lofty::{file::TaggedFileExt, prelude::ItemKey, probe::Probe, tag::Tag};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::Path, thread};
use walkdir::WalkDir;

pub fn list_audio_files() -> Result<Vec<[String; 3]>, ServerFnError> {
    let user_dirs: UserDirs = UserDirs::new().unwrap();
    let audio_dir: &str = user_dirs.audio_dir().unwrap().to_str().unwrap();

    let files: Vec<[String; 3]> = WalkDir::new(audio_dir)
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
                    .unwrap_or("Unknown Album")
                    .to_string();

                Some([title, album, full_path])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(files)
}

pub fn play_music(file_path: &str) {
    let file_path: String = file_path.to_string();

    thread::spawn(move || {
        // _stream must live as long as the sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        let file: BufReader<File> = BufReader::new(File::open(file_path).unwrap());

        let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    });
}
