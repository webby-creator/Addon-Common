use std::sync::LazyLock;

use api::{CmsRowResponse, ListResponse, WrappingResponse};
use eyre::Result;
use global_common::{
    request::CmsQuery,
    uuid::{CollectionName, UuidType},
};
use reqwest::Client;

const MAIN_API_ADDRESS: &'static str = "http://127.0.0.1:5998";
const ADDON_ADDRESS: &'static str = "http://127.0.0.1:5950";

static REQ_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub async fn query_cms_rows(
    uuid: UuidType,
    collection: CollectionName,
    query: CmsQuery,
) -> Result<ListResponse<CmsRowResponse>> {
    match uuid {
        UuidType::Site(uuid) => {
            let resp = REQ_CLIENT
                .get(format!(
                    "{MAIN_API_ADDRESS}/cms/s:{uuid}/schema/{collection}/query?{}",
                    serde_qs::to_string(&query).unwrap()
                ))
                .send()
                .await?
                .json::<WrappingResponse<ListResponse<CmsRowResponse>>>()
                .await?;

            match resp {
                WrappingResponse::Resp(resp) => {
                    return Ok(resp);
                }

                WrappingResponse::Error(v) => {
                    return Err(eyre::eyre!("API Response Error: {}", v.description))?;
                }
            }
        }

        UuidType::Addon(uuid) => {
            let resp = REQ_CLIENT
                .get(format!(
                    "{ADDON_ADDRESS}/addon/{uuid}/schema/{collection}/query?{}",
                    serde_qs::to_string(&query).unwrap()
                ))
                .send()
                .await?
                .json::<WrappingResponse<ListResponse<CmsRowResponse>>>()
                .await?;

            match resp {
                WrappingResponse::Resp(resp) => {
                    return Ok(resp);
                }

                WrappingResponse::Error(v) => {
                    return Err(eyre::eyre!("Addon Response Error: {}", v.description))?;
                }
            }
        }
    }
}

pub async fn get_cms_row_by_id(
    uuid: UuidType,
    collection: CollectionName,
    row_id: &str,
) -> Result<CmsRowResponse> {
    match uuid {
        UuidType::Site(uuid) => {
            let resp = REQ_CLIENT
                .get(format!(
                    "{MAIN_API_ADDRESS}/cms/s:{uuid}/schema/{collection}/row/{row_id}",
                ))
                .send()
                .await?
                .json::<WrappingResponse<CmsRowResponse>>()
                .await?;

            match resp {
                WrappingResponse::Resp(resp) => {
                    return Ok(resp);
                }

                WrappingResponse::Error(v) => {
                    return Err(eyre::eyre!("API Response Error: {}", v.description))?;
                }
            }
        }

        UuidType::Addon(uuid) => {
            let resp = REQ_CLIENT
                .get(format!(
                    "{ADDON_ADDRESS}/addon/{uuid}/schema/{collection}/row/{row_id}",
                ))
                .send()
                .await?
                .json::<WrappingResponse<CmsRowResponse>>()
                .await?;

            match resp {
                WrappingResponse::Resp(resp) => {
                    return Ok(resp);
                }

                WrappingResponse::Error(v) => {
                    return Err(eyre::eyre!("Addon Response Error: {}", v.description))?;
                }
            }
        }
    }
}
