use super::requests::{PageCreationRequest, PageUpdateRequest};
use crate::models::PageModel;
use crate::services::IPageService;
use crate::utils::http_result::ok;
use actix_web::{web, Responder};
use std::sync::Arc;

type PageService = web::Data<Arc<dyn IPageService>>;

pub async fn list_pages(
    page_service: PageService,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let pages = (&page_service).list_pages(&path.1).await?;

    ok(pages)
}

pub async fn get_page(
    page_service: PageService,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let page = (&page_service).get_page(&path.2).await?;

    ok(page)
}

pub async fn create_page(
    page_service: PageService,
    path: web::Path<(String, String)>,
    body: web::Json<PageCreationRequest>,
) -> impl Responder {
    let mut page = PageModel {
        site_version_id: Some(path.1.to_owned()),
        url_path: Some(body.url_path.to_owned()),
        content: Some(body.content.to_owned()),
        ..Default::default()
    };

    (&page_service).add_page(&mut page).await?;

    ok(page)
}

pub async fn update_page(
    page_service: PageService,
    path: web::Path<(String, String, String)>,
    body: web::Json<PageUpdateRequest>,
) -> impl Responder {
    let mut page = PageModel {
        id: Some(path.2.to_owned()),
        url_path: Some(body.url_path.to_owned()),
        content: Some(body.content.to_owned()),
        ..Default::default()
    };

    (&page_service).update_page(&mut page).await?;

    ok(())
}

pub async fn delete_page(
    page_service: PageService,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    (&page_service).delete_page(&path.2).await?;
    ok(())
}
