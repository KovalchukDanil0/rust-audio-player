use std::{error::Error, fs::read_dir, path::PathBuf};

pub fn read_files_with_extension(dir: &str, ext: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let paths: Vec<PathBuf> = read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `csv`
        .filter_map(|path| {
            if path.extension().map_or(false, |file_ext| file_ext == ext) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}
