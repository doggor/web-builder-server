use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageCreationRequest {
    pub url_path: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PageUpdateRequest {
    pub url_path: String,
    pub content: String,
}
