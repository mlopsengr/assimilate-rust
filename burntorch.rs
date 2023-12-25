/*Ported from here: https://raw.githubusercontent.com/nbigaoutte */  
#![forbid(unsafe_code)]

use onnyxruntime::{
    environment::Environment, ndarray::Array, tensor::OrtOwnedTensor, GraphOptimizationLevel, 
    LoggingLevel,
};
use tracing::Level;
use tracing_subsriber::FmtSubscriber;

type Error = Box<dyn std::error::Error>;

pub fn run() -> Result<(), Error> {
    // Setup the example's log level.
    // NOTE: ONNX Runtime's log level is controlled separately when building the environment.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
                            
}
