use walkdir::WalkDir;
use std::path::PathBuf;

pub fn walk_dir(dir: &str) -> Vec<String> {
    let mut paths = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            // Convert PathBuf to string and normalize to use `/` as separator
            let normalized_path = entry.path().display().to_string().replace(std::path::MAIN_SEPARATOR, "/");
            paths.push(normalized_path);
        }
    }

    paths
}
