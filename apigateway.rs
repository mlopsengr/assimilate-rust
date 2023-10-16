use crate::{domain, store, Product};
use lambda_http:{http::StatusCode, IntoResponse, Request, RequestExt, Response};
use serde_json::json;
use tracing::{error, info, instrument, warn};

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

/// Delete a product
#[instrument(skip(store))]
pub async fn delete_product(
    
)