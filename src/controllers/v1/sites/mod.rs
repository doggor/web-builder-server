mod handlers;
mod requests;

use actix_web::web;
use handlers::{create_site, delete_site, get_site, list_sites, update_site};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/sites").route(web::get().to(list_sites)));

    cfg.service(
        web::resource("/sites/{id}")
            .route(web::get().to(get_site))
            .route(web::post().to(create_site))
            .route(web::put().to(update_site))
            .route(web::delete().to(delete_site)),
    );
}
