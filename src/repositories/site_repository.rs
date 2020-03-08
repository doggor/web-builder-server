use crate::models::SiteModel;
use crate::utils::{from_doc, to_doc, CliError};
use async_trait::async_trait;
use chrono::offset::Utc;
use mongodb::options::{FindOptions, UpdateModifications};
use std::result;

type Result<T> = result::Result<T, CliError>;

#[async_trait]
pub trait ISiteRepository: Send + Sync {
    async fn list(&self, from: u32, length: u32) -> Result<Vec<SiteModel>>;
    async fn get(&self, object_id: &str) -> Result<SiteModel>;
    async fn add<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel>;
    async fn update<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel>;
    async fn delete(&self, object_id: &str) -> Result<()>;
}

pub struct SiteRepository {
    collection: mongodb::Collection,
}

impl SiteRepository {
    pub fn new(client: &mongodb::Client) -> SiteRepository {
        SiteRepository {
            collection: client.database("space").collection("sites"),
        }
    }
}

#[async_trait]
impl ISiteRepository for SiteRepository {
    async fn list(&self, from: u32, length: u32) -> Result<Vec<SiteModel>> {
        let cursor = self.collection.find(
            None,
            FindOptions::builder()
                .sort(doc! {
                    "created_at": -1
                })
                .skip(from as i64)
                .limit(length as i64)
                .build(),
        )?;

        let sites: Vec<SiteModel> = cursor
            .filter_map(result::Result::ok)
            .flat_map(from_doc::<SiteModel>)
            .collect();

        Ok(sites)
    }

    async fn get(&self, object_id: &str) -> Result<SiteModel> {
        let result = self.collection.find_one(
            doc! {
                "_id": object_id,
            },
            None,
        )?;

        let document = result.ok_or(CliError::NotFoundError)?;

        Ok(from_doc::<SiteModel>(document)?)
    }

    async fn add<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel> {
        //fill time fields
        model.created_at = Some(Utc::now());
        model.updated_at = Some(Utc::now());

        let result = self.collection.insert_one(to_doc(model)?, None)?;

        model.id = result
            .inserted_id
            .as_object_id()
            .map(|object_id| object_id.to_hex());

        Ok(model)
    }

    async fn update<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel> {
        //update time fields
        model.updated_at = Some(Utc::now());

        let result = self.collection.update_one(
            doc! {
                "_id": model.id.as_ref().unwrap(),
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
