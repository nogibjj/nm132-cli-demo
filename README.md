![CI/CD Pipeline](https://github.com/athletedecoded/aws-candle/actions/workflows/CICD.yml/badge.svg)

# AWS Candle CLI Tool

A Rust CLI tool to support [Candle Cookbook](https://github.com/nogibjj/candle-cookbook/tree/main) on AWS built using the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) 


## Setup

1. Configure an [EC2 instance](https://aws.amazon.com/ec2/instance-types/) depending on your model target and memory requirements. For a GPU Deep Learning enabled instance follow the [Hello, Candle on AWS! tutorial](https://nogibjj.github.io/candle-cookbook/aws/hello-aws.html)

2. Create an AWS IAM User Policy "Candle-Cli" with `AmazonS3FullAccess` and `AmazonEC2FullAccess` permissions

3. Create an access key for your "Candle-Cli" user

4. Create `.env` file and configure with AWS credentials and EC2 Key Pair

```
# $touch .env
AWS_ACCESS_KEY_ID=<YOUR_ACCESS_KEY>
AWS_SECRET_ACCESS_KEY=<YOUR_ACCESS_SECRET>
AWS_DEFAULT_REGION=<YOUR_AWS_REGION>
EC2_KEY=PATH/TO/EC2_KEY_PAIR.pem
```

## CLI Useage

**List Resources**

Args:
* --s3 ~ flag to list s3 resources
* --ec2 ~ flag to list ec2 resources and status
```
# To list simultaneously
$ cargo run list --s3 --ec2
```

**EC2 Instance**

Args:
* --id ~ instance id
* --action:
    * start: start instance
    * stop: stop instance
```
$ cargo run instance --id <INSTANCE_ID> --action <start|stop>
```

**SSH Connect to EC2 Instance**

Quick Tip: first run `cargo run list --ec2` to confirm instance is running!

Args:
* --id ~ instance id
* --mnt ~ mount local model directory to EC2 on launch

```
$ cargo run connect --id <INSTANCE_ID>

# To close SSH tunnel
$ exit

# Make sure to stop your EC2 instance to avoid charges
$ cargo run instance --id <INSTANCE_ID> --action stop
```

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

## Model Sources

1. Prebuilt on EC2
2. Mount from local directory ()
3. Cp from S3
4. Download prebuilt binaries

Once connected to EC2 you can get and run any of the prebuilt binaries [here](https://github.com/athletedecoded/cookbook-binaries/tree/main/binaries). Make sure to run `chmod +x` to give EC2 execution permissions!

```
# Ex
wget -O mistral-cudnn https://github.com/athletedecoded/cookbook-binaries/raw/main/binaries/cudnn/mistral?download=
chmod +x mistral-cudnn
./mistral-cudnn --prompt "who invented the lightbulb"
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
