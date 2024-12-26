use include_dir::{include_dir, Dir};

pub const INDEX_TEMPLATE: &str = include_str!("index.html");

pub const STATIC_DIRECTORY: Dir = include_dir!("static");