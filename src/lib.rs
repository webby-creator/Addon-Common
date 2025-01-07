use std::sync::{LazyLock, RwLock};

use uuid::Uuid;

mod id;
mod registration;
pub mod request;
mod response;
mod structs;

pub use id::*;
pub use registration::*;
pub use response::*;
pub use structs::*;

static CALL_TOKEN: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::default());

// TODO: Security
pub fn register_call_token(value: Uuid) {
    *CALL_TOKEN.write().unwrap() = format!("Basic {value}");
}

pub(crate) fn get_call_token() -> String {
    CALL_TOKEN.read().unwrap().clone()
}
