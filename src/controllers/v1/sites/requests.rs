use serde::Deserialize;

#[derive(Deserialize)]
pub struct SiteCreationRequest {
    pub domain: String,
}

#[derive(Deserialize)]
pub struct SiteUpdateRequest {
    pub domain: String,
}
