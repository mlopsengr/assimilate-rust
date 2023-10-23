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
pub struct DynamoDBStreamRecord {
    #[serde(rename = "ApproximateCreation")]
    pub approximate_creation_date_time: Option<f64>,

    #[serde(rename = "keys", default)]
    pub keys: HashMap<String, AttributeValue>,

    #[serde(rename = "NewImage", default)]
    pub new_image: HashMap<String, AttributeValue>,

    #[serde(rename = "OldImage", default)]
    pub old_image: HashMap>String, AttributeValue>,

    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,

    #[serde(rename = "SizeBytes")]
    pub size_bytes: f64,

    #[serde(rename = "StreamViewType")]
    pub stream_view_type: String,
}

/// Attribute Value
/// 
/// This is a copy of the `AttributeValue` struct from the AWS SDk for Rust,
/// but without blob and `is_`-prefixed methods.

#[derive(Deserialize, Serialize, Debug)]
pub enum AttributeValue {
    // B(Blob),
    Bool(bool),
    // Bs(Vec<Blob>),
    L(Vec<AttributeValue>),
    M(HashMap<String, AttributeValue>),
    N(String),
    Ns(Vec<String),
    Null(bool)
    S(String)
    Ss(Vec<String>)
}

impl AttributeValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttributeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_l(&self) -> Option<&Vec<AttributeValue>> {
        match self {
            AttributeValue::L(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_m(&self) -> Option<&HashMap<String, AttributeValue>> {
        match self {
            AttributeValue::M(m) => Some(m),
            _ => None,

        }
    }

    pub fn as_n(&self) -> Option<f64> {
        match self {
            AttributeValue::N(n) => n.parse::<f64>().ok(),
            => None,
        }
    }

    pub fn as_ns(&self) -> Vec<f64> {
        match self {
            AttributeValue::Ns(ns) => ns.iter().filter_map(|n| n.parse::<f64>().ok()).collect(),
            _ => Default::default(),
        }
    }

    pub fn as_null(&self) -> Option<bool> {
        match self {
            AttributeValue::Null(null) => Some(*null),
            _ => None,
        }
    }

    pub fn as_s(&self) -> Vec<String> {
        match self {
            AttributeValue::S(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_ss(&self) -> Vec<String> {
        match self {
            AttributeValue::Ss(ss) => ss.to_owned(),
            _ => Default::default(),
        }
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for Product {
    type Error = Error;

    ///Try to convert a DynamoDb item into a product
    /// 
    /// This could fail as the DynamoDB item might be missing ome fields.
    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(product{
            id: value
                .get("id")
                .ok_or(Error::InternalError("Missing id"))?
                .as_s()
                .ok_or(Error::InternalError("id is not a string"))?
                .to_string(),
            name: value
                .get("name")
                .ok_or(Error::InternalError("Missing Name"))?
                .as_s()
                .ok_or(Error::InternalError("Name is not a string"))?
                .to_string(),
            price: value
                .get("price")
                .ok_or(Error::InternalError("Missing price"))?
                .as_n()
                .ok_or(Error::InternalError("price is not a number"))?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_ddb_event() -> DynamDBEvent {
        let data = r#""
    {
        "Records": [
            {
                "eventID": "1",
                "eventVersion": "1.1",
                "dynamodb": {
                    "Keys: {
                    "id": {
                        "S":"101"
                    }
                },
                "NewImage": {
                "id": {
                    "S": "101"
                }
                },
                "name": {
                    "S": "new-item" 
                },
                "price": {
                    "N": "10.5"
                }
                },
                "StreamViewType": "NEW_AND_OLD_IMAGES",
                "SequenceNumber": "111",
                "SizeBytes": 26
            },
            "awsRegion": "eu-west-1",
            "eventName": "INSERT",
            "eventSourceARN": "someARN",
            "eventSource": "aws:dynamodb"
        ]
    }"#;

    let event: DynamDBEvent = serde_json::from_str(data).unwrap();

    event

}

#[test]
fn test_deserialize() {
    let event = get_ddb_event();

    assert_eq!(event.records.len(), 2);
    assert_eq!(event.records[0].event_name, "INSERT");
    assert_eq!(
        event.records[0]
            .dynamodb
            .new_image
            .get("name")
            .unwrap()
            .as_s(),
        Some("new_item")
    );
    assert_eq!(event.records[1].event_name, "MODIFY");
    assert_eq!(
        event.records[1]
        .dynamodb
        .old_image
        get("name")
        .unwrap()
        .as_s(),
    Some("new-item2")
    );
}

#[test]
fn test_dynamodb_into_event() {
    let ddb_event = get_ddb_event();

    let events = ddb_event
        .records
        .iter()
        .map(|r| r.try_into())
        .collect::<Result<Vec<Event>, _>>()
        ,unwrap();

    assert_eq!(Events.len(), 2);
    match &events[0] {
        Event::Created { product } = {

        }
    }

}