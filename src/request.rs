use std::sync::LazyLock;

use api::{CmsRowResponse, ListResponse, WrappingResponse};
use eyre::Result;
use global_common::{
    request::CmsQuery,
    uuid::{CollectionName, UuidType},
};
use reqwest::Client;

const ADDON_ADDRESS: &'static str = "http://127.0.0.1:5950";

static REQ_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub async fn query_cms_rows(
    uuid: UuidType,
    collection: CollectionName,
    query: CmsQuery,
) -> Result<ListResponse<CmsRowResponse>> {
    match uuid {
        UuidType::Site(uuid) => {
            // let res = REQ_CLIENT
            //     .get("http://localhost:5941/instance/query")
            //     .send()
            //     .await?;

            // Ok(Json(res.json().await?))

            todo!("Site UUID Query")
        }

        UuidType::Addon(addon_uuid) => {
            let resp = REQ_CLIENT
                .get(format!(
                    "{ADDON_ADDRESS}/addon/{addon_uuid}/schema/{collection}/query?{}",
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
