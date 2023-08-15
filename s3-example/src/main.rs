use std::{fs::File, io::{Read, Write,}};

use s3::{bucket::Bucket, error::S3Error};
use s3::creds::Credentials;

type MyResult<T> = Result<T, S3Error>;

// create config functions for easily switching between minio and s3
async fn upload_object(file_id: &str, path: &str) -> MyResult<()> {
    let cred = Credentials::default()?;
    let region = "us-east-1".parse()?;
    let bucket = Bucket::new("atomic-metameeee", region, cred).unwrap();

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
