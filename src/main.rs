// use aws_config::meta::region::RegionProviderChain;
use clap::Parser;
mod s3;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "AWS Candle CLI",
    after_help = "Example: aws-candle"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    List {
        #[clap(short, long)]
        bucket: Option<String>,
    },
    Create {
        #[clap(short, long)]
        bucket: String,
    },
    Upload {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        filepath: String,
    },
    Delete {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        key: Option<String>,
    },
    Get {
        #[clap(short, long)]
        bucket: String,
        #[clap(short, long)]
        key: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    // Load AWS credentials from .env file
    dotenv::dotenv().ok();
    let shared_config = aws_config::load_from_env().await;
    let client = s3::s3client(shared_config).await.unwrap();
    // Match on subcommand
    match args.command {
        Some(Commands::Create { bucket }) => {
            let bucket_region = s3::check_region().await.unwrap();
            s3::create_bucket(&client, &bucket, &bucket_region)
                .await
                .unwrap();
        }
        Some(Commands::List { bucket }) => match bucket {
            Some(bucket) => {
                s3::list_objects(&client, &bucket).await.unwrap();
            }
            None => {
                s3::list_buckets(&client).await.unwrap();
            }
        },
        Some(Commands::Upload { bucket, filepath }) => {
            s3::upload_object(&client, &bucket, &filepath)
                .await
                .unwrap();
        }
        Some(Commands::Delete { bucket, key }) => match key {
            Some(key) => {
                s3::delete_object(&client, &bucket, &key).await.unwrap();
            }
            None => {
                s3::delete_bucket(&client, &bucket).await.unwrap();
            }
        },
        Some(Commands::Get { bucket, key }) => {
            s3::get_object(&client, &bucket, &key).await.unwrap();
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
