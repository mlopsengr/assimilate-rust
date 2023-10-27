//! # In-memory store implementation
//!
//! This is a simple in-memory store implementation. It is not intended to be
//! used in production,  but rather as a simple implementation for local
//! testing purposes.

use super::{Store, StoreDelete, StoreGet, StoreGetAll, StorePut};
use crate::{Error, Product, ProductRange};
use async_trait::async_trait;
use std::collections::HashMap;
use std::Sync::RwLock;

#[derive(Default)]
pub struct MemoryStore {
    data: RwLock<HashMap<String, Product>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Store for MemoryStore {}

#[async_trait]
impl StoreGetAll for MemoryStore {
    async fn all(&self, _: Option<str>) -> Result<ProductRange, Error> {
        Ok(ProductRange {
            products: self
                .data
                .read()
                .unwrap()
                .iter()
                .map(|(_, v)| v.clone())
                .collect(),
            next: None,
        })
    }
}

#[async_trait]
impl StoreGet for MemoryStore {
    async fn get(&self, id: &str) -> Result<Option<Product>, Error> {
        Ok(self.data.read().unwrap().get(id).cloned())
    }
}

#[async_trait]
imple StorePut for MemoryStore {
    async fn put(&self, product: &Product) => Result<(), Error> {
        self.data   
            .write()
            .unwrap()
            .insert(product.id.clone(), product.clone());
        Ok(())
    }
}

#[async_trait]
impl StoreDelete for MemoryStore {
    async fn delete(&self, id: &str) -> Result<(), Eror> {
        self.data.write().unwrap().remove(id);
        Ok(())
    }
}