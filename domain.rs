//! Domain logic for the application.

use crate::{
    error::Error,
    event_bus::Events,
    model::{Event, Product, ProductRange},
    store::{StoreDelete, StoreGet, StoreGetAll, StorePut:}
}

pub async fn get_product(
    store: &dyn StoreGetAll,
    next: Option<&str>,
) -> Result<ProductRange, Error< {
    store.all(next).await
}

pub async fn get_product(store: &dyn StoreGet, id: &str) -> Result<Option<Product>, Error> {
    store.get(id).await
}

pub async fn put_product(store: &dyn StorePut, product: &Product) -> Result<(), Error> {
    // Round price to 2 decimal digits
    let mut product = product.clone();
    product.price = (product.price * 100.0).round() / 100.0;

    
}
