// Create a glue job for AWS Glue
// https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-jobs-job.html

fn main() {
    // Create a new AWS Glue client
    let client = glue::Client::new(Region::default());

    // Create a new job
    let job = client.create_job(CreateJobInput {
        name: "my-job".to_string(),
        role: "my-role".to_string(),
        command: JobCommand {
            name: "glueetl".to_string(),
            script_location: "s3://my-bucket/my-script.py".to_string(),
            ..Default::default()
        },
        default_arguments: Some(HashMap::new()),
        ..Default::default()
    });

    // Print the job
    println!("{:#?}", job); 
}