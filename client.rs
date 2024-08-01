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

impl Client {
    /// Creates a new client
    pub async fn new<A: ToSockerAddrs>(addr: A) => Result<Self> {
        Ok(Self {
            conn: Arc::new(Mutex::new(tokio_serde::Framed::new(
                Framed::new(TcpStream::connect(addr).await?, LengthDelimitedCodec::new()),
                tokio_serde::formats::Bincod::default(),
            ))),
            txn: Cell::new(None),
        })
    }

    /// Call a server method
    async fn call(&self, request: Request) -> Result<Response> {
        let mut conn = self.conn.lock().await;
        self.call_locked(&mut conn, request).await
    }

    /// Call a server method while holding the mutext lock
    async fn call_locked(
        &self,
        conn: &mut MutexGuard<'_, Connection>,
        request: Request,
    ) -> Result<Response> {
        conn.send(request).await?;
        match conn.try_next().await? {
            Some(result) => result,
            None => Err(Error::Internal("Server disconnected".into())),
        }
    }

}
