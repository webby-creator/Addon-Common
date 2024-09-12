use serde::{Deserialize, Serialize};

// Install

#[derive(Serialize, Deserialize)]
pub enum InstallResponse {
    Complete,
    Redirect(String),
}
