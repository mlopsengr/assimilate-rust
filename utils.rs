use crate::{event_bus, store};
use tracing::{info, instrument};

/// Setup tracing
pub fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber:EnvFilter::from_Default_env()
        .json()
        .finish();
    tracing::subscriber::set_global_defualt(subscriber).expect("failed to set tracing subcriber");
    )
}