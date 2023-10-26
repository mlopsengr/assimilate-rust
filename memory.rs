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
