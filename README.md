![CI/CD Pipeline](https://github.com/athletedecoded/aws-candle/actions/workflows/CICD.yml/badge.svg)

# AWS Candle CLI Tool

A Rust CLI tool to support [Candle Cookbook](https://github.com/nogibjj/candle-cookbook/tree/main) on AWS built using the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) 


## Setup

1. Create an AWS IAM User Policy with `AmazonS3FullAccess` and `AmazonEC2FullAccess` permissions

2. Create an access key for your user

3. Create `.env` file and configure with AWS credentials and EC2 Key Pair

```
# $touch .env
AWS_ACCESS_KEY_ID=<YOUR_ACCESS_KEY>
AWS_SECRET_ACCESS_KEY=<YOUR_ACCESS_SECRET>
AWS_DEFAULT_REGION=<YOUR_AWS_REGION>
EC2_KEY=PATH/TO/EC2_KEY_PAIR.pem
```

## Useage

**List Resources**

Args:
* --s3 ~ list s3 resources
* --ec2 ~ list ec2 resources and status
```
$ cargo run list --s3 --ec2
```

**EC2 Instance**

Args:
* --id ~ instance id
* --action:
    * start: start instance
    * stop: stop instance
    * connect: ssh into instance -- tip: first run `cargo run list --ec2` to check instance is running
```
$ cargo run instance --id <INSTANCE_ID> --action <start|stop|connect>
```

Once connected to EC2 you can get and run any of the prebuilt binaries [here](https://github.com/athletedecoded/cookbook-binaries/tree/main/binaries). Make sure to run `chmod +x` to give EC2 execution permissions!

```
# Ex
wget -O mistral-cudnn https://github.com/athletedecoded/cookbook-binaries/raw/main/binaries/cudnn/mistral?download=
chmod +x mistral-cudnn
./mistral-cudnn --prompt "who invented the lightbulb"
```

To close SSH tunnel run `exit`. Be sure to stop your instance when done.

**S3 Buckets**

Args:
* --name ~ bucket name
* --action:
    * list: list objects in the bucket
    * create: create new bucket with name --name
    * delete: delete bucket with name --name
```
$ cargo run bucket --name <BUCKET_NAME> --action <list|create|delete>
```

**S3 Objects**

Args
* --bucket ~ bucket name -- NB: Will create bucket if DNE
* --key ~ object key or path/to/file for --action upload
* --action:
    * upload: upload local file as bucket object
    * delete: delete object with key --key
    * get: download object with key --key to ./root

```
$ cargo run object --bucket <bucket_name> --key <object_key> --action <upload|delete|get>"
# ex: cargo run object --bucket my-bucket --key ./test/test.png --action upload
# ex: cargo run object --bucket my-bucket --key test.png --action get
```

## CI/CD

Github Actions configured in [.github/workflows/CICD.yml](.github/workflows/CICD.yml)

**Build Executable**
```
$ make release
```

## References

* [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
* [AWS Toolkit Credential Profile](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-where)
* [AWS Credentials for VS Code](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/setup-credentials.html)
