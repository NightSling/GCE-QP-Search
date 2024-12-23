use rocket::response::content::RawHtml;

use crate::templates;


#[get("/")]
pub fn index() -> rocket::response::content::RawHtml<&'static str> {
    RawHtml(templates::INDEX_TEMPLATE)
}