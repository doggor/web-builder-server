#[macro_use]
extern crate bson;

mod controllers;
mod models;
mod repositories;
mod services;
mod utils;

use actix_web::{App, HttpServer};
use controllers::v1::sites::config as site_config;
use mongodb;
use repositories::{ISiteRepository, SiteRepository};
use services::{ISiteService, SiteService};
use std::sync::Arc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //crate mongodb client
    let mongodb_client = mongodb::Client::with_uri_str("mongodb://localhost:27017").unwrap();

    //init repositories
    let site_repository: Arc<dyn ISiteRepository> = Arc::new(SiteRepository::new(&mongodb_client));

    //init services
    let site_service: Arc<dyn ISiteService> = Arc::new(SiteService::new(site_repository.clone()));

    //create app for each thread and start serving
    HttpServer::new(move || {
        App::new()
            .data(site_service.clone())
            .configure(site_config)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
