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

**List all S3 buckets**
```
$ cargo run list
```

**List all objects in a specified S3 bucket**
```
$ cargo run list --bucket <bucket_name>
# ex: cargo run list --bucket ids721
```

**Create new S3 bucket**
```
$ cargo run create --bucket <bucket_name>
# ex: cargo run create --bucket ids721
```

**Upload an object to an S3 bucket**

*NB: Will create bucket if DNE*
```
$ cargo run upload --bucket <bucket_name> --filepath <path_to_file>
# ex: cargo run upload --bucket ids721 --filepath ./test/test.png
```

**Delete an object from an S3 bucket**
```
$ cargo run delete --bucket <bucket_name> --key <object_key>
# ex: cargo run delete --bucket ids721 --key test.png
```

**Delete an empty S3 bucket**
```
$ cargo run delete --bucket <bucket_name>
# ex: cargo run delete --bucket ids721
```

**Get an object from an S3 bucket**
```
$ cargo run get --bucket <bucket_name> --key <object_key>
# ex: cargo run get --bucket ids721 --key test.jpg
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
* [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)
