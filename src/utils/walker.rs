use walkdir::WalkDir;

pub fn walk_dir(dir: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            paths.push(entry.path().display().to_string());
        }
    }
    paths
}