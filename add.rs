// calculator functions 
//Add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Substract two numbers
pub fn substract(a: i32, b: i32) -> i32 {
    a - b
}

//Multiply two number
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

//Divide two numbers
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

use rand_core::{Error, RngCore};
use crate::distributions::uniform::{SampleRange, SampleUniform};
use crate::distributions::{self, Distribution, Standard};
use core::num::Wrapping;
use core::{mem, slice};

pub trait Rng: RngCore {
    fn gen<T>(&mut self) -> T
    where Standard: Distribution<T> {
        Standard.sample(self)
    }
}

fn main() {
    // @MLOPs_engineer 
    let client = lambda::Client::new(Region::UsEast1); // Creates a client
    let result = create_lambda_function(&client);
    match result {
        Ok(_) => println!("Lambda function created"),
        Err(e) => println!("Error creating lambda function: {:?}", e),
    }

}

fn create_lambda_function(client: &lambda::Client) -> Result<(), lambda::Error> {
    let request = lambda::CreateFunctionRequest {
        code: lambda::FunctionCode {
            zip_file: Some(include_bytes!("../target/lambda/release/bootstrap.zip").to_vec()),
            ..Default::default()
        },
        function_name: "calculator".to_string(),
        handler: "bootstrap".to_string(),
        role: "arn:aws:iam::123456789012:role/lambda_basic_execution".to_string(),  // dummy role
        runtime: "provided".to_string(),
        ..Default::default()
    };

    client.create_function(request).sync()
}



