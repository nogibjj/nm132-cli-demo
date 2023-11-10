![CI/CD Pipeline](https://github.com/athletedecoded/aws-candle/actions/workflows/CICD.yml/badge.svg)

# AWS Candle CLI Tool

A Rust CLI tool to support [Candle Cookbook](https://github.com/nogibjj/candle-cookbook/tree/main) on AWS built using the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) 


## Setup

1. Create an AWS IAM User Policy with `AmazonS3FullAccess` and `AmazonEC2FullAccess` permissions

2. Create an access key for your user

3. Create `.env` file and configure with AWS credentials

```
# $touch .env
AWS_ACCESS_KEY_ID=<YOUR_ACCESS_KEY>
AWS_SECRET_ACCESS_KEY=<YOUR_ACCESS_SECRET>
AWS_DEFAULT_REGION=<YOUR_AWS_REGION>
```


## Useage

**List Resources**
```
$ cargo run list <--s3> <--ec2>
```

**Launch/Stop EC2 Instance**
```
$ cargo run instance --id <INSTANCE_ID> --action <start|stop>
```

**S3 Buckets**
```
$ cargo run bucket --name <BUCKET_NAME> --action <list|create|delete>
```

**S3 Objects**

*NB: Will create bucket if DNE. For "upload" --key is path/to/object*

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
