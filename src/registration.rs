use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    structs::{MemberPartial, WebsitePartial},
    MemberUuid, WebsiteUuid,
};

// Install

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNewJson {
    pub instance_id: Uuid,

    pub website_id: WebsiteUuid,
    pub owner_id: MemberUuid,

    pub member: MemberPartial,
    pub website: WebsitePartial,
}

#[derive(Serialize, Deserialize)]
pub enum InstallResponse {
    Complete,
    Redirect(String),
}
