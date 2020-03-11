use serde::Deserialize;

#[derive(Deserialize)]
pub struct SiteVersionCreationRequest {
    pub name: String,
    pub favicon_path: String,
}

#[derive(Deserialize)]
pub struct SiteVersionUpdateRequest {
    pub name: String,
    pub favicon_path: String,
}
