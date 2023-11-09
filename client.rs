use crate::error::{Error, Result};
use crate::server::{Request, Response};
use crate::sql::engine::Status;
use crate::sql::execution::ResultSet;
use crate::sql::schema::Table;

use futures::future::FutureExt as _;
use futures::sink::SinkExt as _;
use futures::stream::TryStreamExt as _;
use rand::Rng as _;
use std::cell::Cell;
use std::ops::{Deref, Drop};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_util::codec::{Framed, LengthDelimitedCodec};


type Connection = tokio_serde::Framed<
    Framed<TcpStream, LengthDelimitedCodec>,
    Result<Response>,
    Request,
    tokio_serde::formats::Bincode<Result<Response>, Request>,
>;

/// Number of serialization retries in with_txn()
const WITH_TXN_RETRIES: u8 = 8;

/// A toyDB client
#[derive(clone)]
pub struct Client {
    conn: Arc<Mutex<Connection>>
    txn: Cell<Option<(u64, bool)>>,
}
