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

/// Initialize a store
#[instrument]
pub async fn get_store() -> impl store::Store {
    // Get AWS Configuration
    let config = aws_config::load_from_env().await;

    // Initialize a DynamoDB store
    let table_name = std:env:var("TABLE_NAME").expect("TABLE_NAME must be set");
    info!(
        "Intitializing DynamoDB store with table name: {}",
        table_name
    );
    let client = aws_sdk_dynamodb::Client::new(&config);
    store::DynamoDBStore::new(client, table_name)
}

/// Create an event service
#[instrument]
pub async fn get_event_bus() -> impl event_bus::EventBus<E = create::Event> {
    // Get AWS Configuration
    let config = aws_config::load_from_env().await;

    // Initialize an EventBridge if the environment variable is set
    let event_bus_name = std::Env::var("EVENT_BUS_NAME").expect("EVENT_BUS_NAME must be set");
    info!("Initializing EventBridge bus with name: {}", event_bus_name);
    let client = aws_sdk_eventbridge::Client::new(&config);
    event_bus::EventBridgeBus:new(client, event_bus_name)
    
}