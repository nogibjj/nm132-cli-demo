use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Error};
use std::path::Path;
use std::process;
use tokio::fs::File;
use tokio::io::copy;

// Create S3 client
pub async fn s3client() -> Result<Client, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

/* -----------------------------
    BUCKET FNXNS
-------------------------------- */

// List all buckets
pub async fn list_buckets(client: &Client) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets();
    let num_buckets = buckets.len();
    println!("Found {num_buckets} buckets.");
    println!();
    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    Ok(())
}

// Check if bucket exists
pub async fn bucket_exists(client: &Client, bucket_name: &str) -> Result<bool, Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets();
    for bucket in buckets {
        if bucket.name().unwrap_or_default() == bucket_name {
            return Ok(true);
        }
    }
    Ok(false)
}

// Create new bucket
pub async fn create_bucket(client: &Client, bucket: &str) -> Result<(), Error> {
    // Check if bucket exists
    let exists = bucket_exists(client, bucket).await?;
    if exists {
        println!("Bucket {bucket} already exists.");
        process::exit(1);
    }
    let region = std::env::var("AWS_DEFAULT_REGION").unwrap();
    match region.as_str() {
        "us-east-1" => {
            let _resp = client
                .create_bucket()
                .bucket(bucket)
                .send()
                .await?;
        }
        _ => {
            let constraint = BucketLocationConstraint::from(region.as_str());
            let bucket_config = CreateBucketConfiguration::builder()
                .location_constraint(constraint)
                .build();
            let _resp = client
                .create_bucket()
                .bucket(bucket)
                .create_bucket_configuration(bucket_config)
                .send()
                .await?;
        }
    }
    println!("Created bucket: {bucket}");
    Ok(())
}

// Delete empty bucket
pub async fn delete_bucket(client: &Client, bucket: &str) -> Result<(), Error> {
    let exists = bucket_exists(client, bucket).await?;
    if !exists {
        println!("Bucket {bucket} does not exist.");
        process::exit(1);
    }
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents();
    let num_objects = objects.len();
    if num_objects != 0 {
        println!("Bucket {bucket} is not empty. Cannot delete.");
        process::exit(1);
    }
    client.delete_bucket().bucket(bucket).send().await?;
    println!("Empty bucket {bucket} deleted.");

    Ok(())
}

/* -----------------------------
    OBJECT FNXNS
--------------------------------*/

// List objects in bucket
pub async fn list_objects(client: &Client, bucket: &str) -> Result<(), Error> {
    // Check if bucket exists
    let exists = bucket_exists(client, bucket).await?;
    if !exists {
        println!("Bucket {bucket} does not exist.");
        process::exit(1);
    }

    // If exists, list objects
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents();
    let num_objects = objects.len();

    println!("Found {num_objects} objects in bucket {bucket}");
    for object in objects {
        println!("{}", object.key().unwrap_or_default());
    }

    Ok(())
}

// Put object in bucket
pub async fn upload_object(client: &Client, bucket: &str, filepath: &str) -> Result<(), Error> {
    // if bucket doesn't exist, create it
    if !bucket_exists(client, bucket).await? {
        create_bucket(client, bucket).await?;
    }

    let body = ByteStream::from_path(Path::new(filepath)).await;
    let key = Path::new(filepath).file_name().unwrap().to_str().unwrap();
    match body {
        Ok(b) => {
            let _resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;
            println!("Uploaded {key} to {bucket}");
        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{e}");
            process::exit(1);
        }
    }

    Ok(())
}

// Delete object from bucket
pub async fn delete_object(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    // Check if bucket exists
    let exists = bucket_exists(client, bucket).await?;
    if !exists {
        println!("Bucket {bucket} does not exist.");
        process::exit(1);
    }

    // Check key exists in bucket
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents();
    let mut key_exists = false;
    for object in objects {
        if object.key().unwrap_or_default() == key {
            key_exists = true;
        }
    }
    if !key_exists {
        println!("Key {key} does not exist in bucket {bucket}");
        process::exit(1);
    }
    // Delete object
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    println!("Object {key} deleted from bucket {bucket}.");

    Ok(())
}

pub async fn get_object(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    // Check key exists in bucket
    let resp = client.list_objects_v2().bucket(bucket).send().await?;
    let objects = resp.contents();
    let mut key_exists = false;
    for object in objects {
        if object.key().unwrap_or_default() == key {
            key_exists = true;
        }
    }
    if !key_exists {
        println!("Key {key} does not exist in bucket {bucket}");
        process::exit(1);
    }
    // Get object
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    // Get image as byte stream from response body
    let fpath = format!("./{}", key);
    let mut img_stream = resp.body.into_async_read();
    // Create a file to write the image data to
    let mut tmp_file = File::create(&fpath).await.unwrap();
    // Copy the image data into the file
    let _file_msg = copy(&mut img_stream, &mut tmp_file).await.unwrap();
    println!("Object downloaded to {fpath}.");
    Ok(())
}