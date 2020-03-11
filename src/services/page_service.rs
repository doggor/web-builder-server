use crate::models::PageModel;
use crate::repositories::IPageRepository;
use crate::utils::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IPageService: Send + Sync {
    async fn list_pages(&self, site_version_id: &str) -> Result<Vec<PageModel>>;
    async fn get_page(&self, id: &str) -> Result<PageModel>;
    async fn add_page<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel>;
    async fn update_page<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel>;
    async fn delete_page(&self, id: &str) -> Result<()>;
}

pub struct PageService {
    page_repo: Arc<dyn IPageRepository>,
}

impl PageService {
    pub fn new(page_repo: Arc<dyn IPageRepository>) -> Self {
        Self { page_repo }
    }
}

#[async_trait]
impl IPageService for PageService {
    async fn list_pages(&self, site_version_id: &str) -> Result<Vec<PageModel>> {
        self.page_repo.list(site_version_id).await
    }

    async fn get_page(&self, id: &str) -> Result<PageModel> {
        self.page_repo.get(id).await
    }

    async fn add_page<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel> {
        self.page_repo.add(model).await
    }

    async fn update_page<'a>(&self, model: &'a mut PageModel) -> Result<&'a mut PageModel> {
        self.page_repo.update(model).await
    }

    async fn delete_page(&self, id: &str) -> Result<()> {
        self.page_repo.delete(id).await
    }
}
