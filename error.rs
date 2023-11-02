use aws_sdk_dynamodb::model::AttributeValue;
use aws_smithy_http::REsult::SdkError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Erro {
    InitError(&'static str),
}