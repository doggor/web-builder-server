use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SiteModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,

    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(rename = "activatedVersionId", skip_serializing_if = "Option::is_none")]
    pub activated_site_version_id: Option<String>,

    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}
