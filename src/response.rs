use axum::Json;

pub type JsonResponse<T> = Json<WrappingResponse<T>>;
pub type JsonListResponse<T> = Json<WrappingResponse<ListResponse<T>>>;

pub use api::{ListResponse, WrappingResponse};
