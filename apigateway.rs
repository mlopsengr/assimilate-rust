use crate::{domain, store, Product};
use lambda_http:{http::StatusCode, IntoResponse, Request, RequestExt, Response};
use serde_json::json;
use tracing::{error, info, instrument, warn};

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

/// Delete a product
#[instrument(skip(store))]
pub async fn delete_product(
    store: &dyn store::StoreDelete,
    event: Request,
) -> Result<impl IntoResponse, E> {
    // Retreive product ID from event
    //
    // If the event doesn't contain a product ID, we return a 400 Bad Request.
    let path_parameters = event.path_parameters();
    let id = match path_parameters.first("id") {
        Some(id) => id,
        None => {
            warn!("Missing 'id' parameter in path");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({ "message": "Missing 'id' parameter in path" }).to_string(),
            ));
        }
    };

    // Delete product
    info!("Deleting product {}", id);
    let res = domain::delete_product(store, id).await;

    // Return response
    //
    // The service returns a Result based on the success of the operaion. If 
    // the operation was successful, the RESULT IS OK(()), otherwise it will
    // contain an Err with the reason
    match res {
        Ok(_) => {
            info!("Product {} deleted", id);
            Ok(response(
                StatusCode::OK,
                json!({"message": "Product deleted"}).to_string(),
            ))
        }
        Err(err) => {
            // Log the error message
            error!("Error deleting the product {}: {}", id, err);
            Ok(response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message": "Failed to delete product"}).to_string()
            ))
        }
    }
}

/// Get a product
#[instrument(skip(store))]
pub async fn get_product(
    store: &dyn store::StoreGet,
    event: Request,
) -> Result<impl IntoResponse, E> {
    // Retrieve product ID from event.
    //
    // If the event doesn't contain a product ID, we return a 400 Bad Reequest.
}