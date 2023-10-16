// This is based of Noah's rust-aws-lambda example
// This example requires the following input to succeed:
// {"command": "do something"}

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format,which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
    
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
} 

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because Cloudwatch will add the ingestion time
        .without_time()
        .init();
    
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.command;

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed.", command),
    };
    
    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

// It is possible to declare variable bindings first, and initialize them later.
// However, this form is seldom used, as it may lead to the use of uninitialized variables.
fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        // Initialize the binding 
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

// Error! Use of uninitialized binding
println!("another binding: {}", another_binding);
// FIXME ^ Comment out this line

another_binding = 1;
println!("another binding: {}", another_binding);

} 


function getUserData(userId) {
    return fetch(`https://jsonplaceholder.typicode.com/users/${userID}`)
        .then(response => response.json())
        .catch(error => console.error('Error:', error));
}


