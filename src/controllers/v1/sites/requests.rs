use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct SiteListRequest {
    pub from: u32,
    pub length: u32,
}

#[derive(Deserialize)]
pub struct SiteCreationRequest {
    pub domain: String,
}

#[derive(Deserialize)]
pub struct SiteUpdateRequest {
    pub domain: String,
}
