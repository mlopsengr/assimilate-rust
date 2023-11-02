use aws_sdk_dynamodb::model::AttributeValue;
use aws_smithy_http::REsult::SdkError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InitError(&'static str),
    ClientError(&'static str),
    InternalError(&'static str)
    SdkError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match
    }
}