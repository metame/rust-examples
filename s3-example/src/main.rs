use std::{fs::File, io::{Error, Read, Write,}};

use s3::{bucket::Bucket, error::S3Error};
use s3::creds::Credentials;

type MyResult<T> = Result<T, S3Error>;

/*
#[allow(unused)]
fn minio() {
    builder.bucket("new-bucket");
    builder.endpoint("http://192.168.1.80:9000");
    builder.access_key_id("minioadmin");
    builder.secret_access_key("minioadmin");
}
*/

fn aws_builder() -> Bucket {
    Bucket::new("atomic-metameeee", "us-east-1".parse().unwrap(), Credentials::default().unwrap()).unwrap()
}


// create config functions for easily switching between minio and s3
async fn upload_object(file_id: &str, path: &str) -> MyResult<()> {
    let bucket = aws_builder();

    let mut file = File::open(path).expect("Unable to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    bucket.put_object(&format!("{}/{}", "uploads", &file_id), &buf).await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut file = File::create(&"/tmp/test").unwrap();
    file.write_all("it works!".as_bytes()).unwrap();
    match upload_object(&"it-works", &"/tmp/test").await {
        Ok(()) => println!("Object uploaded!"),
        Err(error) => println!("Error: {:?}", error),
    }
}
