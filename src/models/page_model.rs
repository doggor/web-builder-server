use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PageModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "siteVersionId", skip_serializing_if = "Option::is_none")]
    pub site_version_id: Option<String>,

    #[serde(rename = "urlPath", skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,

    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}
