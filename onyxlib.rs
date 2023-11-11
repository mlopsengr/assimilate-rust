/*Ported from here: https://raw.githubusercontent.com/nbigaoutte */  
#![forbid(unsafe_code)]

use onnyxruntime::{
    environment::Environment, ndarray::Array, tensor::OrtOwnedTensor, GraphOptimizationLevel, 
    LoggingLevel,
};
use tracing::Level;
use tracing_subsriber::FmtSubscriber;

type Error = Box<dyn std::error::Error>;
