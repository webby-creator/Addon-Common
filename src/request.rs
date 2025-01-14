use std::{collections::HashMap, sync::LazyLock};

use api::{
    CmsCreateResponse, CmsRowResponse, ListResponse, SchemaTag, SimpleValue, WrappingResponse,
};
use eyre::Result;
use global_common::{
    request::{CmsCreate, CmsCreateDataColumnTag, CmsQuery},
    uuid::{CollectionName, UuidType},
};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, ClientBuilder, IntoUrl,
};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::{get_call_token, WebsiteUuid};

pub use api::form::*;

const MAIN_API_ADDRESS: &'static str = "http://127.0.0.1:5998";
const ADDON_ADDRESS: &'static str = "http://127.0.0.1:5950";

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

static REQ_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    ClientBuilder::default()
        .default_headers({
            let mut headers = HeaderMap::new();

            let mut auth_value = HeaderValue::from_str(&get_call_token()).unwrap();
            auth_value.set_sensitive(true);
            headers.insert(AUTHORIZATION, auth_value);

            headers
        })
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
});

pub async fn create_cms_collection(uuid: UuidType, cms: CmsCreate) -> Result<CmsCreateResponse> {
    match uuid {
        UuidType::Site(uuid) => {
            let resp = REQ_CLIENT
                .post(format!("{MAIN_API_ADDRESS}/cms/s:{uuid}/new"))
                .json(&cms)
                .send()
                .await?
                .json::<WrappingResponse<CmsCreateResponse>>()
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
                .post(format!("{ADDON_ADDRESS}/addon/{uuid}/schema/new",))
                .json(&cms)
                .send()
                .await?
                .json::<WrappingResponse<CmsCreateResponse>>()
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

pub async fn create_cms_collection_tags(
    uuid: UuidType,
    coll: CollectionName,
    column_id: &str,
    create: CmsCreateDataColumnTag,
) -> Result<SchemaTag> {
    match uuid {
        UuidType::Site(uuid) => {
            let resp = REQ_CLIENT
                .post(format!(
                    "{MAIN_API_ADDRESS}/cms/s:{uuid}/data/{coll}/column/{column_id}/tag"
                ))
                .json(&create)
                .send()
                .await?
                .json::<WrappingResponse<SchemaTag>>()
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
                .post(format!(
                    "{ADDON_ADDRESS}/addon/{uuid}/schema/{coll}/column/{column_id}/tag",
                ))
                .json(&create)
                .send()
                .await?
                .json::<WrappingResponse<SchemaTag>>()
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

pub async fn query_cms_rows(
    uuid: UuidType,
    collection: CollectionName,
    query: CmsQuery,
) -> Result<ListResponse<CmsRowResponse>> {
    match uuid {
        UuidType::Site(uuid) => {
            let resp = REQ_CLIENT
                .get(format!(
                    "{MAIN_API_ADDRESS}/cms/s:{uuid}/data/{collection}/query?{}",
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
                    "{MAIN_API_ADDRESS}/cms/s:{uuid}/data/{collection}/row/{row_id}",
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

pub async fn import_data_row(
    uuid: UuidType,
    collection: CollectionName,
    rows: HashMap<String, SimpleValue>,
) -> Result<String> {
    import_data_rows(
        uuid,
        collection,
        rows.into_iter().map(|(k, v)| (k, vec![v])).collect(),
    )
    .await
}

pub async fn import_data_rows(
    uuid: UuidType,
    collection: CollectionName,
    rows: HashMap<String, Vec<SimpleValue>>,
) -> Result<String> {
    match uuid {
        UuidType::Site(uuid) => {
            let resp = REQ_CLIENT
                .post(format!(
                    "{MAIN_API_ADDRESS}/cms/s:{uuid}/data/{collection}/import",
                ))
                .json(&rows)
                .send()
                .await?
                .json::<WrappingResponse<String>>()
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
                .post(format!(
                    "{ADDON_ADDRESS}/addon/{uuid}/schema/{collection}/import",
                ))
                .json(&rows)
                .send()
                .await?
                .json::<WrappingResponse<String>>()
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

pub async fn create_website_form(
    website_id: WebsiteUuid,
    opts: CreateWebsiteForm,
) -> Result<WebsiteForm> {
    let resp = post_json_response::<WebsiteForm>(
        format!("{MAIN_API_ADDRESS}/form/{website_id}/create",),
        &opts,
    )
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

pub async fn create_website_form_action(
    website_id: WebsiteUuid,
    form_id: Uuid,
    action: FormAction,
) -> Result<()> {
    let resp = post_json_response::<String>(
        format!("{MAIN_API_ADDRESS}/form/{website_id}/{form_id}/actions"),
        &action,
    )
    .await?;

    match resp {
        WrappingResponse::Resp(_resp) => {
            return Ok(());
        }

        WrappingResponse::Error(v) => {
            return Err(eyre::eyre!("API Response Error: {}", v.description))?;
        }
    }
}

async fn post_json_response<D: DeserializeOwned>(
    url: impl IntoUrl,
    json: &impl Serialize,
) -> Result<WrappingResponse<D>> {
    let resp = REQ_CLIENT.post(url).json(json).send().await?;

    if resp.status().is_success() {
        return Ok(resp.json().await?);
    }

    let content = resp.text().await?;

    if let Ok(resp) = serde_json::from_str(&content) {
        Ok(resp)
    } else {
        // Whatever the error response is. It's not a valid json
        Ok(WrappingResponse::error(content))
    }
}
