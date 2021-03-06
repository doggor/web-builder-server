use crate::models::SiteVersionModel;
use crate::utils::{from_doc, to_doc, CliError, Result};
use async_trait::async_trait;
use chrono::offset::Utc;
use mongodb::options::{FindOptions, UpdateModifications};
use std::result;

#[async_trait]
pub trait ISiteVersionRepository: Send + Sync {
    async fn list(&self, site_id: &str) -> Result<Vec<SiteVersionModel>>;
    async fn get(&self, object_id: &str) -> Result<SiteVersionModel>;
    async fn add<'a>(&self, model: &'a mut SiteVersionModel) -> Result<&'a mut SiteVersionModel>;
    async fn update<'a>(&self, model: &'a mut SiteVersionModel)
        -> Result<&'a mut SiteVersionModel>;
    async fn delete(&self, object_id: &str) -> Result<()>;
}

pub struct SiteVersionRepository {
    collection: mongodb::Collection,
}

impl SiteVersionRepository {
    pub fn new(client: &mongodb::Client) -> Self {
        Self {
            collection: client.database("space").collection("site_versions"),
        }
    }
}

#[async_trait]
impl ISiteVersionRepository for SiteVersionRepository {
    async fn list(&self, site_id: &str) -> Result<Vec<SiteVersionModel>> {
        let cursor = self.collection.find(
            doc! {
                "site_id": site_id,
            },
            FindOptions::builder()
                .sort(doc! {
                    "created_at": -1,
                })
                .build(),
        )?;

        let site_versions: Vec<SiteVersionModel> = cursor
            .filter_map(result::Result::ok)
            .flat_map(from_doc::<SiteVersionModel>)
            .collect();

        Ok(site_versions)
    }

    async fn get(&self, object_id: &str) -> Result<SiteVersionModel> {
        let result = self.collection.find_one(
            doc! {
                "_id": object_id,
            },
            None,
        )?;

        let document = result.ok_or(CliError::NotFoundError)?;

        Ok(from_doc::<SiteVersionModel>(document)?)
    }

    async fn add<'a>(&self, model: &'a mut SiteVersionModel) -> Result<&'a mut SiteVersionModel> {
        model.created_at = Some(Utc::now());
        model.updated_at = Some(Utc::now());

        let result = self.collection.insert_one(to_doc(model)?, None)?;

        model.id = result
            .inserted_id
            .as_object_id()
            .map(|object_id| object_id.to_hex());

        Ok(model)
    }

    async fn update<'a>(
        &self,
        model: &'a mut SiteVersionModel,
    ) -> Result<&'a mut SiteVersionModel> {
        model.updated_at = Some(Utc::now());

        let result = self.collection.update_one(
            doc! {
                "_id": model.id.as_ref().ok_or(CliError::InvalidDataError("id".to_owned()))?,
            },
            UpdateModifications::Document(to_doc(model)?),
            None,
        )?;

        if result.modified_count > 0 {
            Ok(model)
        } else {
            Err(CliError::NotFoundError)
        }
    }

    async fn delete(&self, object_id: &str) -> Result<()> {
        let result = self.collection.delete_one(
            doc! {
                "_id": object_id
            },
            None,
        )?;

        if result.deleted_count > 0 {
            Ok(())
        } else {
            Err(CliError::NotFoundError)
        }
    }
}
