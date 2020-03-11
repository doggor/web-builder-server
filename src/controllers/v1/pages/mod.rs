mod handlers;
mod requests;

use actix_web::web;
use handlers::{create_page, delete_page, get_page, list_pages, update_page};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/sites/{site_id}/versions/{version_id}/pages")
            .route(web::get().to(list_pages)),
    );

    cfg.service(
        web::resource("/sites/{site_id}/versions/{version_id}/pages/{page_id}")
            .route(web::get().to(get_page))
            .route(web::post().to(create_page))
            .route(web::put().to(update_page))
            .route(web::delete().to(delete_page)),
    );
}
