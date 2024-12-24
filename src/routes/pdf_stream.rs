use std::path::PathBuf;

use rocket::State;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

use crate::utils::process_file::CaseInsensitiveGet;
use crate::AppState;


#[get("/<paper..>", rank=1)]
pub async fn stream_paper(state: &State<AppState>,  paper: PathBuf) -> Result<NamedFile, NotFound<&'static str>> {
    let paper = match paper.to_str() {
        Some(paper) => paper.to_lowercase(),
        None => return Err(NotFound("Failed to join the Paper PathBuf")),
    };
    let paper = crate::utils::santizier::sanitize_input(&paper);
    let paper = state.files.get_case_insensitive(&paper);
    let paper = match paper {
        Some(paper) => paper,
        None => return Err(NotFound("Paper not found in the database")),
    };
    match NamedFile::open(paper).await {
        Ok(file) => Ok(file),
        Err(_) => Err(NotFound("Paper not found in file system")),
    }
}
