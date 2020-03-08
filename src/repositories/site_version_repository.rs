use crate::models::SiteVersionModel;
use crate::utils::{CliError, from_doc, to_doc};
use async_trait::async_trait;
use chrono::offset::Utc;
use mongodb::options::{FindOptions, UpdateModifications};
use std::result;

type Result<T> = result::Result<T, CliError>;

#[async_trait]
pub trait ISiteVersionRepository: Send + Sync {
    async fn list(&self, site_id: &str) ->Result<Vec<SiteVersionModel>>;
    async fn get(&self, object_id, &str) -> Result<SiteVersionModel>;
    async fn add<'a>(&self, model: &'a mut SiteVersionModel) -> Result<&'a mut SiteVersionModel>;
    async fn update<'a>(&self, model: &'a mut SiteVersionModel) -> Result<&'a mut SiteVersionModel>;
    async fn delete(&self, object_id: &str) -> Result<()>;
}
