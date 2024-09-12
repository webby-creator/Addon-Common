use axum::Json;
use serde::{Deserialize, Serialize};

pub type JsonResponse<T> = Json<WrappingResponse<T>>;
pub type JsonListResponse<T> = Json<WrappingResponse<ListResponse<T>>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
}

impl<T> ListResponse<T> {
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            offset: 0,
            limit: 0,
            total: 0,
        }
    }

    pub fn all(value: Vec<T>) -> Self {
        Self {
            offset: 0,
            limit: value.len(),
            total: value.len(),
            items: value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleValue<V>(pub V);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum WrappingResponse<V> {
    Resp(V),
    Error(ApiErrorResponse),
}

impl<V> WrappingResponse<V> {
    pub fn okay(value: V) -> Self {
        Self::Resp(value)
    }

    pub fn error<S: Into<String>>(value: S) -> Self {
        Self::Error(ApiErrorResponse::new(value))
    }

    pub fn ok(self) -> std::result::Result<V, ApiErrorResponse> {
        match self {
            Self::Resp(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }

    pub fn as_ok(&self) -> std::result::Result<&V, &ApiErrorResponse> {
        match self {
            Self::Resp(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }

    pub fn map<N, F: Fn(V) -> N>(self, func: F) -> WrappingResponse<N> {
        match self {
            Self::Resp(v) => WrappingResponse::Resp(func(v)),
            Self::Error(e) => WrappingResponse::Error(e),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub struct ApiErrorResponse {
    pub description: String,
}

impl ApiErrorResponse {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self {
            description: value.into(),
        }
    }
}

impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api Error Occurred: {}", self.description)
    }
}
