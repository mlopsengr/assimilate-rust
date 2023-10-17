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
    let path_parameters = even.path_parameters();
    let id = match path_parameters.first("id") {
        Some(id) => id,
        None => {
            warn!("Missing 'id' parameter in path"); 
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Missing 'id' parameter in path"}).to_string(),
            ));
        }
    };

    // Retrieve product
    info!("Fetching product {}", id);
    let product = domain::get_product(store, id).await;
    
    // Retuen response
    //
    // Since the service returns an `Option` within a `Result`, there are threee
    // potential scenarios: the product exists, it doesn't exist, or there was
    // as error.
    Ok(match product {
        // Product exists
        Ok(Some(product)) +> response(StatusCode::OK, json!(product).to_string()),
        // Product doesn't exist
        Ok(None) => {
            warn!("Product not found: {}", id);
            response(
                StatusCode::NOT_FOUND,
                json!({"message": "product not found"}).to_string(),
            )
        }
        // Error
        Err(err) => {
            error!("Error fetching product: {}", err);
            response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message": "Error fetching product"}).to_string(),
            )
        }
        // Error
    })
}

/// Retrieve products
#[instrument(skip(store))]
pub async fn get_products(
    store: &dyn store::StoreGetAll,
    _event: Request,
) -> Result<impl IntoResponse, E> {
    // Retreive products
    // TODO: Add pagination
    let res = domain::get_products(store, None).await;

    // Return response
    Ok(match res {
        // Retrun a list of products
        Ok(res) => response(StatusCode::OK, json!(res).to_string()),
        // Return an error
        Err(err) => {
            error!("Something went wrong: {:?}", err);
            response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "message": format!("Something went wroong: {:?}", err) }).to_string(),
            )
        }
    })
}

/// Put a product
#[instrument(skip(store))]
pub aysnc fn put_product(
    store: &dyn store::StorePut,
    event: Request,
) -> Result<impl IntoResponse, E> {
    // Retrieve product ID from event.
    //
    // If the event doesn't contain a product ID, we return a 400 Bad Request.
    let path_parameters = event.path_parameters();
    let id = match path_parameters.first("id") {
        Some(id) => id,
        None => {
            warn!("Missing 'id' parameter in path");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Missing 'id' parametere in path" }).to_string(),
            ));
        }
    };

    // Read product from request
    let product: Product = match event.payload() {
        Ok(Some(product)) => product,
        Ok(None) => {
            warn!("Missing product in request body");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Missing product in request body"}).to_string(),
            ));
        }
        Err(err) => {
            warn!("Failed to parse product from request body: {}", err);
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message": "Failed to parse product from request body"}).to_string(),
            ));
        }
    }
}