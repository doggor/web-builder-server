#[macro_use]
extern crate bson;

mod controllers;
mod models;
mod repositories;
mod services;
mod utils;

use actix_web::{App, HttpServer};
use controllers::v1::{
    pages::config as page_config, site_versions::config as site_version_config,
    sites::config as site_config,
};
use mongodb;
use repositories::{PageRepository, SiteRepository, SiteVersionRepository};
use services::{
    IPageService, ISiteService, ISiteVersionService, PageService, SiteService, SiteVersionService,
};
use std::sync::Arc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //crate mongodb client
    let mongodb_client = mongodb::Client::with_uri_str("mongodb://localhost:27017").unwrap();

    //init repositories
    let site_repository = Arc::new(SiteRepository::new(&mongodb_client));
    let site_version_repository = Arc::new(SiteVersionRepository::new(&mongodb_client));
    let page_repository = Arc::new(PageRepository::new(&mongodb_client));

    //init services
    let site_service = Arc::new(SiteService::new(site_repository.clone()));
    let site_version_service = Arc::new(SiteVersionService::new(site_version_repository.clone()));
    let page_service = Arc::new(PageService::new(page_repository.clone()));

    //create app for each thread and start serving
    HttpServer::new(move || {
        App::new()
            .data(site_service.clone() as Arc<dyn ISiteService>)
            .data(site_version_service.clone() as Arc<dyn ISiteVersionService>)
            .data(page_service.clone() as Arc<dyn IPageService>)
            .configure(site_config)
            .configure(site_version_config)
            .configure(page_config)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
