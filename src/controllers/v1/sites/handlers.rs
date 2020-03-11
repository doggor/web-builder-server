use super::requests::{SiteCreationRequest, SiteListRequest, SiteUpdateRequest};
use crate::models::SiteModel;
use crate::services::ISiteService;
use crate::utils::http_result::ok;
use actix_web::{web, Responder};
use std::sync::Arc;

type SiteService = web::Data<Arc<dyn ISiteService>>;

pub async fn list_sites(
    site_service: SiteService,
    query: web::Query<SiteListRequest>,
) -> impl Responder {
    let sites = (&site_service).list_sites(query.from, query.length).await?;

    ok(sites)
}

pub async fn get_site(site_service: SiteService, path: web::Path<(String,)>) -> impl Responder {
    let site = (&site_service).get_site(&path.0).await?;

    ok(site)
}

pub async fn create_site(
    site_service: SiteService,
    body: web::Json<SiteCreationRequest>,
) -> impl Responder {
    let mut site = SiteModel {
        domain: Some(body.domain.to_owned()),
        ..Default::default()
    };

    (&site_service).add_site(&mut site).await?;

    ok(site)
}

pub async fn update_site(
    site_service: SiteService,
    path: web::Path<(String,)>,
    body: web::Json<SiteUpdateRequest>,
) -> impl Responder {
    let mut site = SiteModel {
        id: Some(path.0.to_owned()),
        domain: Some(body.domain.to_owned()),
        ..Default::default()
    };

    (&site_service).update_site(&mut site).await?;

    ok(())
}

pub async fn delete_site(site_service: SiteService, path: web::Path<(String,)>) -> impl Responder {
    (&site_service).delete_site(&path.0).await?;
    ok(())
}
