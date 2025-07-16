use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{MemberUuid, WebsiteUuid};

// TODO: Create Permissions Macro to convert Full Struct -> Partial Struct

#[derive(Serialize, Deserialize)]
pub struct MemberPartial {
    pub uuid: MemberUuid,

    pub role: i64,

    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct WebsitePartial {
    // pub owner_uuid: MemberUuid,
    pub public_id: WebsiteUuid,

    pub name: String,
    /// If URL starts with '/' it is relative to the domain
    pub url: Option<String>,
    pub theme_id: i32,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
