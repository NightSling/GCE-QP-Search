#![feature(allocator_api)]

// When the `system-alloc` feature is used, use the System Allocator
#[cfg(feature = "system-alloc")]
mod allocator {
    use std::heap::System;

    #[global_allocator]
    pub static mut THE_ALLOC: System = System;
}

// When the `system-alloc` feature is not used, do nothing,
// retaining the default functionality (using jemalloc)
#[cfg(not(feature = "system-alloc"))]
mod allocator {
    #[allow(dead_code)]
    pub static THE_ALLOC: () = ();
}

#[allow(unused_imports)]
use allocator::THE_ALLOC;

use std::collections::HashMap;
use utils::{process_file, walker::walk_dir};

#[macro_use]
extern crate rocket;


mod routes;
mod templates;
mod utils;

struct AppState {
    files: HashMap<String, String>,
    folder_path: String,
}

#[launch]
fn rocket() -> _ {
    let static_folder = utils::env::get_env_var("STATIC_FOLDER", "pdfs");

    // Walk dir static_folder, save as vec to the path.
    let files = walk_dir(&static_folder);
    let files_map: HashMap<String, String> = HashMap::new();

    let mut app_state = AppState {
        files: files_map,
        folder_path: static_folder,
    };

    files.iter().for_each(|file| {
        process_file::process_file(&mut app_state.files, file);
    });    

    rocket::build()
        .mount("/viewer", routes![routes::viewer::get_pdf_viewer])
        .mount("/", routes![routes::index::index])
        .mount("/pdf", routes![routes::pdf_stream::stream_paper])
        .manage(app_state)
}