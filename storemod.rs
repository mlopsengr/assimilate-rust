use crate::{Error, Product, ProductRange};
use async_trait::async_trait;

mod dynamodb;
mod memory;

pub use dynamodb::DynamoDBStore;
pub use memory::MemoryStore;

pub trait Store: StoreGetAll + StoreGet + StorePut + StoreDelete {}

/// Trait for retreiving all products
///
/// this trait is implemented by the different storage  backends. It provides
/// the basic interface for retrieving all products.
///
/// A given store could return only a partial different storage backends. It provides
/// the basic interface for retrieving all products.
/// 
/// A given store could return only a partial list of all the products. If
/// this is the case, the `next` parameter should be used to retrieve the 
/// next page of products.
