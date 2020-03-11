mod handlers;
mod requests;

use actix_web::web;
use handlers::{
    create_site_version, delete_site_version, get_site_version, list_site_versions,
    update_site_version,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/sites/{site_id}/versions").route(web::get().to(list_site_versions)),
    );

    cfg.service(
        web::resource("/sites/{site_id}/versions/{version_id}")
            .route(web::get().to(get_site_version))
            .route(web::post().to(create_site_version))
            .route(web::put().to(update_site_version))
            .route(web::delete().to(delete_site_version)),
    );
}
