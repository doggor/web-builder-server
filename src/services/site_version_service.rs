use crate::models::SiteVersionModel;
use crate::repositories::ISiteVersionRepository;
use crate::utils::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ISiteVersionService: Send + Sync {
    async fn list_site_versions(&self, site_id: &str) -> Result<Vec<SiteVersionModel>>;
    async fn get_site_version(&self, id: &str) -> Result<SiteVersionModel>;
    async fn add_site_version<'a>(
        &self,
        model: &'a mut SiteVersionModel,
    ) -> Result<&'a mut SiteVersionModel>;
    async fn update_site_version<'a>(
        &self,
        model: &'a mut SiteVersionModel,
    ) -> Result<&'a mut SiteVersionModel>;
    async fn delete_site_version(&self, id: &str) -> Result<()>;
}

pub struct SiteVersionService {
    site_version_repo: Arc<dyn ISiteVersionRepository>,
}

impl SiteVersionService {
    pub fn new(site_version_repo: Arc<dyn ISiteVersionRepository>) -> Self {
        Self { site_version_repo }
    }
}

#[async_trait]
impl ISiteVersionService for SiteVersionService {
    async fn list_site_versions(&self, site_id: &str) -> Result<Vec<SiteVersionModel>> {
        self.site_version_repo.list(site_id).await
    }

    async fn get_site_version(&self, id: &str) -> Result<SiteVersionModel> {
        self.site_version_repo.get(id).await
    }

    async fn add_site_version<'a>(
        &self,
        model: &'a mut SiteVersionModel,
    ) -> Result<&'a mut SiteVersionModel> {
        self.site_version_repo.add(model).await
    }

    async fn update_site_version<'a>(
        &self,
        model: &'a mut SiteVersionModel,
    ) -> Result<&'a mut SiteVersionModel> {
        self.site_version_repo.update(model).await
    }

    async fn delete_site_version(&self, id: &str) -> Result<()> {
        self.site_version_repo.delete(id).await
    }
}
