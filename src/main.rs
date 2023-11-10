use clap::{Args, Parser, Subcommand};
mod s3;
mod ec2;

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

#[derive(Subcommand)]
enum Commands {
    // Download(DownloadArgs),  // download binaries from s3 or glfs
    List(ListArgs),             // list resources i.e. s3 buckets, ec2 instances
    Instance(InstanceArgs),     // start/stop ec2 instance by id
    Bucket(BucketArgs),         // create, list items, or delete a bucket
    Object(ObjectArgs),         // upload, delete or get object from a bucket
}

#[derive(Args)]
struct DownloadArgs {
   #[arg(short, long)]
   src: Option<String>,
   #[arg(short, long)]
   target: Option<String>,
   #[arg(short, long)]
   model: Option<String>,   // "all" will sync entire folder
}

#[derive(Args)]
struct InstanceArgs {
   #[arg(short, long)]
   id: Option<String>,
   #[arg(short, long)]
   action: String,
}

#[derive(Args)]
struct ListArgs {
    #[arg(short, long)]
    s3: bool,
    #[arg(short, long)]
    ec2: bool,
}

#[derive(Args)]
struct BucketArgs {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    action: String,
}

#[derive(Args)]
struct ObjectArgs {
    #[arg(short, long)]
    bucket: String, // bucket name
    #[arg(short, long)]
    key: String, // object name or local path
    #[arg(short, long)]
    action: String, // object key
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // Load AWS credentials from .env file
    dotenv::dotenv().ok();
    let s3client = s3::s3client().await.unwrap();
    let ec2client = ec2::ec2client().await.unwrap();
    // Match on subcommand
    match cli.command {
        // List Commands
        Some(Commands::List(args)) => {
            if args.s3 {
                s3::list_buckets(&s3client).await.unwrap();
            }
            if args.ec2 {
                ec2::list_instances(&ec2client).await.unwrap();
            }
            // if all args are false, give help
            if !args.s3 && !args.ec2 {
                println!("Useage: cargo run list --s3 --ec2");
            }

        }
        // Instance Commands
        Some(Commands::Instance(args)) => {
            match args.action.as_str() {
                // start instance
                "start" => {
                    ec2::start_instance(&ec2client, &args.id.unwrap())
                        .await
                        .unwrap();
                }
                // stop instance
                "stop" => {
                    ec2::stop_instance(&ec2client, &args.id.unwrap())
                        .await
                        .unwrap();
                }
                _ => {
                    println!("Useage: cargo run instance --id <instance_id> --action <start|stop>");
                }
            }
        }
        // S3 Bucket Commands
        Some(Commands::Bucket(args)) => {
            match args.action.as_str() {
                // create bucket
                "create" => {
                    s3::create_bucket(&s3client, &args.name)
                        .await
                        .unwrap();
                }
                // delete bucket
                "delete" => {
                    s3::delete_bucket(&s3client, &args.name).await.unwrap();
                }
                // list bucket contents
                "list" => {
                    s3::list_objects(&s3client, &args.name).await.unwrap();
                }
                _ => {
                    println!("Useage: cargo run bucket --name <bucket_name> --action <create|delete|list>");
                }
            }
        }
        // S3 Object Commands
        Some(Commands::Object(args)) => {
            match args.action.as_str() {
                // upload object
                "upload" => {
                    s3::upload_object(&s3client, &args.bucket, &args.key)
                        .await
                        .unwrap();
                }
                // delete object
                "delete" => {
                    s3::delete_object(&s3client, &args.bucket, &args.key)
                        .await
                        .unwrap();
                }
                // get object
                "get" => {
                    s3::get_object(&s3client, &args.bucket, &args.key)
                        .await
                        .unwrap();
                }
                _ => {
                    println!("Useage: cargo run object --bucket <bucket_name> --key <object_key> --action <upload|delete|get>");
                }
            }
        }

        None => {
            println!("No subcommand was used");
        }
    }
}
