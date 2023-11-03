//! Domain logic for the application.

use crate::{
    error::Error,
    event_bus::Events,
    model::{Event, Product, ProductRange},
    store::{StoreDelete, StoreGet, StoreGetAll, StorePut:}
}

