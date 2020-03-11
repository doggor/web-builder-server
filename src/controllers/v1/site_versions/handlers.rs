use super::requests::{SiteVersionCreationRequest, SiteVersionUpdateRequest};
use crate::models::SiteVersionModel;
use crate::services::ISiteVersionService;
use crate::utils::http_result::ok;
use actix_web::{web, Responder};
use std::sync::Arc;

type SiteVersionService = web::Data<Arc<dyn ISiteVersionService>>;

pub async fn list_site_versions(
    site_version_service: SiteVersionService,
    path: web::Path<(String,)>,
) -> impl Responder {
    let site_versions = (&site_version_service).list_site_versions(&path.0).await?;

    ok(site_versions)
}

pub async fn get_site_version(
    site_version_service: SiteVersionService,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let site_version = (&site_version_service).get_site_version(&path.1).await?;

    ok(site_version)
}

pub async fn create_site_version(
    site_version_service: SiteVersionService,
    body: web::Json<SiteVersionCreationRequest>,
) -> impl Responder {
    let mut site_version = SiteVersionModel {
        name: Some(body.name.to_owned()),
        favicon_path: Some(body.favicon_path.to_owned()),
        ..Default::default()
    };

    (&site_version_service)
        .add_site_version(&mut site_version)
        .await?;

    ok(site_version)
}

pub async fn update_site_version(
    site_version_service: SiteVersionService,
    path: web::Path<(String, String)>,
    body: web::Json<SiteVersionUpdateRequest>,
) -> impl Responder {
    let mut site_version = SiteVersionModel {
        id: Some(path.1.to_owned()),
        name: Some(body.name.to_owned()),
        favicon_path: Some(body.favicon_path.to_owned()),
        ..Default::default()
    };

    (&site_version_service)
        .update_site_version(&mut site_version)
        .await?;

    ok(())
}

pub async fn delete_site_version(
    site_version_service: SiteVersionService,
    path: web::Path<(String, String)>,
) -> impl Responder {
    (&site_version_service).delete_site_version(&path.1).await?;
    ok(())
}
