use crate::models::PageModel;
use crate::utils::{from_doc, to_doc, CliError, Result};
use async_trait::async_trait;
use chrono::offset::Utc;
use mongodb::options::{FindOptions, UpdateModifications};
use std::result;

#[async_trait]
pub trait IPageRepository: Send + Sync {
    async fn list(&self, site_version_id: &str) -> Result<Vec<PageModel>>;
    async fn get(&self, object_id: &str) -> Result<PageModel>;
    async fn add<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel>;
    async fn update<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel>;
    async fn delete(&self, object_id: &str) -> Result<()>;
}

pub struct PageRepository {
    collection: mongodb::Collection,
}

impl PageRepository {
    pub fn new(client: &mongodb::Client) -> Self {
        Self {
            collection: client.database("space").collection("pages"),
        }
    }
}

#[async_trait]
impl IPageRepository for PageRepository {
    async fn list(&self, site_version_id: &str) -> Result<Vec<PageModel>> {
        let cursor = self.collection.find(
            doc! {
                "site_version_id": site_version_id,
            },
            FindOptions::builder()
                .sort(doc! {
                    "url_path": 1,
                })
                .build(),
        )?;

        let pages: Vec<PageModel> = cursor
            .filter_map(result::Result::ok)
            .flat_map(from_doc::<PageModel>)
            .collect();

        Ok(pages)
    }

    async fn get(&self, object_id: &str) -> Result<PageModel> {
        let result = self.collection.find_one(
            doc! {
                "_id": object_id,
            },
            None,
        )?;

        let document = result.ok_or(CliError::NotFoundError)?;

        Ok(from_doc::<PageModel>(document)?)
    }

    async fn add<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel> {
        model.created_at = Some(Utc::now());
        model.updated_at = Some(Utc::now());

        let result = self.collection.insert_one(to_doc(model)?, None)?;

        model.id = result
            .inserted_id
            .as_object_id()
            .map(|object_id| object_id.to_hex());

        Ok(model)
    }

    async fn update<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel> {
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
                "_id": object_id,
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
