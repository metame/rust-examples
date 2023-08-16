use std::{fs::File, io::{Error, Read, Write,}};

use opendal::{Operator, services::S3};

#[allow(unused)]
fn minio() -> S3 {
    let mut builder = S3::default();
    builder.bucket("atomic");
    builder.endpoint("http://192.168.1.80:9000");
    builder.access_key_id("minioadmin");
    builder.secret_access_key("minioadmin");
    builder
}

fn aws_builder() -> S3 {
    let mut builder = S3::default();
    builder.endpoint("https://s3.amazonaws.com");
    builder.bucket("atomic-metameeee");
    builder.region("us-east-1");
    builder
}

async fn upload_object(file_id: &str, path: &str) -> Result<(), Error> {
    let builder = aws_builder();

    let op: Operator = Operator::new(builder)?.finish();

    let mut file = File::open(path).expect("Unable to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    op.write(&format!("{}/{}", "uploads", &file_id), buf).await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut file = File::create(&"/tmp/test").unwrap();
    file.write_all("it works!".as_bytes()).unwrap();
    match upload_object(&"it-works-opendal", &"/tmp/test").await {
        Ok(()) => println!("Object uploaded!"),
        Err(error) => println!("Error: {}", error),
    }
}
