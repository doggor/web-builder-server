use crate::models::SiteModel;
use crate::repositories::ISiteRepository;
use crate::utils::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ISiteService: Send + Sync {
    async fn list_sites(&self, from: u32, length: u32) -> Result<Vec<SiteModel>>;
    async fn get_site(&self, id: &str) -> Result<SiteModel>;
    async fn add_site<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel>;
    async fn update_site<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel>;
    async fn delete_site(&self, id: &str) -> Result<()>;
}

pub struct SiteService {
    site_repo: Arc<dyn ISiteRepository>,
}

impl SiteService {
    pub fn new(site_repo: Arc<dyn ISiteRepository>) -> Self {
        Self { site_repo }
    }
}

#[async_trait]
impl ISiteService for SiteService {
    async fn list_sites(&self, from: u32, length: u32) -> Result<Vec<SiteModel>> {
        self.site_repo.list(from, length).await
    }

    async fn get_site(&self, id: &str) -> Result<SiteModel> {
        self.site_repo.get(id).await
    }

    async fn add_site<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel> {
        self.site_repo.add(model).await
    }

    async fn update_site<'a>(&self, model: &'a mut SiteModel) -> Result<&'a mut SiteModel> {
        self.site_repo.update(model).await
    }

    async fn delete_site(&self, id: &str) -> Result<()> {
        self.site_repo.delete(id).await
    }
}
