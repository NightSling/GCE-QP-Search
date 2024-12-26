use std::path::PathBuf;

use rocket::http::ContentType;

use crate::templates::STATIC_DIRECTORY;


#[derive(Responder)]
#[response(status = 200)]
pub struct StaticResponse {
    inner: &'static [u8],
    header: ContentType,
}

#[get("/<file..>", rank=1)]
pub fn get_pdf_viewer(file: PathBuf) -> StaticResponse {
    let file = match file.to_str() {
        Some(file) => file,
        None => return StaticResponse {
            inner: "File not found".as_bytes(),
            header: ContentType::Plain,
        },
    };

    let file = file.replace(std::path::MAIN_SEPARATOR, "/");

    let file = match STATIC_DIRECTORY.get_file(&file) {
        Some(file) => file,
        None => return StaticResponse {
            inner: "File not found".as_bytes(),
            header: ContentType::Plain
        },
    };
    dbg!(file);

    let extension = file.path().extension();
    extension.map_or_else(|| {
        StaticResponse {
            inner: "File has no extension".as_bytes(),
            header: ContentType::Plain,
        }
    }, |ext| ext.to_str().map_or_else(|| {
        StaticResponse {
            inner: "Extension is not a valid UTF-8 sequence".as_bytes(),
            header: ContentType::Plain,
        }
    }, |ext| {
        StaticResponse {
            inner: file.contents(),
            header: ContentType::from_extension(ext).unwrap_or({
                ContentType::Plain
            }),
        }
    }))
}
