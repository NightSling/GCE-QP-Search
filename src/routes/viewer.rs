use std::path::PathBuf;

use rocket::http::ContentType;


#[derive(Responder)]
#[response(status = 200)]
pub struct StaticResponse {
    content: &'static str,
    header: ContentType,
}

#[get("/<file..>", rank=1)]
pub fn get_pdf_viewer(file: PathBuf) -> StaticResponse {
    let file = match file.to_str() {
        Some(file) => file.to_lowercase(),
        None => return StaticResponse {
            content: "File not found",
            header: ContentType::Plain,
        },
    };

    match file.as_str() {
        "web/viewer.html" => StaticResponse {
            content: crate::templates::PDF_VIEWER_HTML,
            header: ContentType::HTML,
        },
        "web/viewer.mjs" => StaticResponse {
            content: crate::templates::PDF_VIEWER_JS,
            header: ContentType::JavaScript,
        },
        "web/viewer.css" => StaticResponse {
            content: crate::templates::PDF_VIEWER_CSS,
            header: ContentType::CSS,
        },
        "build/pdf.worker.mjs" => StaticResponse {
            content: crate::templates::PDF_WORKER_JS,
            header: ContentType::JavaScript,
        },
        "build/pdf.mjs" => StaticResponse {
            content: crate::templates::PDF_LIB_JS,
            header: ContentType::JavaScript,
        },
        _ => StaticResponse {
            content: "File not found",
            header: ContentType::Plain,
        },
    }
    
}