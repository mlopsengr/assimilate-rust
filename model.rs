//! Dynamo DB Event models
//!
//! Models for the DynamoDB event entrypoint.
//!
//! We cannot use the models provided by the AWS SDK for Rust, as they do not
//! implement the `serde::Serialize` and `serde::Deserialize` traits.

use crate::{
    model::{Event, Product},
    Error,
};
use serde::{Deserialize, Serialize}
use std::collections::HashMap;


#[derive(Deserialize, Serialize, Debug)]
pub struct DynamoDBEvent {
    #[serde(rename = "Records")]
    pub records: Vec<DynamoDBRecord>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DynamoDBEvent {
    #[serde(rename = "awsRegion")]
    pub aws_region: String,

    #[serde(rename = "DynamoDB")]
    pub dynamodb: DynamoDBStreamRecord,

    #[serde(rename = "eventID")]
    pub event_id: String,

    #[serde(rename = "eventName")]
    pub event_name: String,

    #[serde(rename = "eventSource")]
    pub event_source = String,

    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,

    #[serde(rename = "EventVersion")]
    pub event_version: String,

}

impl TryFrom<&DynamoDBRecord> for Event {
    type Error = Error;


    /// Try converting a DynamoDB record to an event.
    fn try_from(value: &DynamoDBRecord) -> Result<Self, Self::Error> {
        match value.event_name.as_str() {
            "INSERT" => {
                let product = (&value.dynamodb.new_image).try_into()?;
                Ok(Event::Created { product })
            }
            "MODIFY" => {
                let old = (&value.dynamodb.old_image).try_into()?;
                let new = (&value.dynamodb.new_image).try_into()?;
                Ok(Event::Updated { old, new })
            }
            _ => Err(Error::InternalError("Unknown event type")), 
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]