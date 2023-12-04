# Cargo Lambda Prototype

- Adapted from https://www.cargo-lambda.info/

## Tools Installation for Windows

- Install scoop
  - https://scoop.sh/
- Install cargo-lambda
  - scoop bucket add cargo-lambda https://github.com/cargo-lambda/scoop-cargo-lambda
  - scoop install cargo-lambda/cargo-lambda
- Install the AWS CLI

## Usage

- Start a Lambda server on your localhost
```
cd cargo-lambda-prototype/
cargo lambda watch
```
- Test the Lambda function using your browser
  - http://localhost:9000/?name=World
  - http://localhost:9000/lambda-url/cargo-lambda-prototype/?name=World
- Open a new command-line terminal
- Invoke the Lambda function from the command line using example input data
  - Note the DEBUG output showing the location of the cached example input file
```
cargo lambda -v invoke \
  --data-example apigw-request cargo-lambda-prototype \
  --output-format json
```
- Edit the cached example input file to change the name
  - Look for the name property under multiValueQueryStringParameters
- Run the cargo lambda invoke command again
  - The output should show the changed name

## Build

- You must build your Lambda before you can deploy it
- Build a release for the ARM64 architecture
```
cargo lambda build --release --arm64
```

## Deploy

- Activate your AWS access key via the AWS Console
- Deploy the Lambda to the cloud
  - This will automatically generate a role for you
  - Note the generated URL
```
cargo lambda deploy --enable-function-url
```
- Test
  - Example URL: https://\[abc123].lambda-url.\[region].on.aws/?name=World

## Undeploy

- Get the Lambda function details
  - You will need the name of the role for the Lambda function in a later step
  - This was the role that was automatically created for you when you deployed
```
aws lambda get-function --function-name cargo-lambda-prototype
```
- Delete the Lambda function
```
aws lambda delete-function --function-name cargo-lambda-prototype
```
- List the policies attached to the role
  - Note the ARN for the policy
```
aws iam list-attached-role-policies --role-name cargo-lambda-role-[UUID]
```
- Detach the managed policy from the role
  - You cannot delete the role until you detach the policy
```
aws iam detach-role-policy \
  --role-name cargo-lambda-role-[UUID] \
  --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
```
- Delete the role
```
aws iam delete-role --role-name cargo-lambda-role-[UUID]
```
- Deactivate your AWS access key via the AWS Console

## Deploy With a Pre-existing Lambda Execution Role

- Activate your AWS access key via the AWS Console
- Validate the CloudFormation (CFn) template file
```
aws cloudformation validate-template --template-body file://role-template.yaml
```
- Create a CloudFormation (CFn) stack that defines a Lambda execution role
```
aws cloudformation create-stack \
  --capabilities CAPABILITY_NAMED_IAM \
  --stack-name cargo-lambda-prototype \
  --template-body file://role-template.yaml
```
- Get the Amazon Resource Name (ARN) for the created role
  - You might have to wait a bit until the stack has been created
```
aws iam get-role --role-name cargo-lambda-prototype
```
- Deploy the Lambda
  - Use the role ARN
```
cargo lambda deploy --enable-function-url --iam-role [ROLE-ARN]
```

## Undeploy With a Pre-existing Lambda Execution Role

- Delete the Lambda function
```
aws lambda delete-function --function-name cargo-lambda-prototype
```
- Wait a bit for the Lambda function to be deleted
- Delete the CFn stack with the Lambda execution role
```
aws cloudformation delete-stack --stack-name cargo-lambda-prototype
```
- Deactivate your AWS access key via the AWS Console

## Deploy With SAM

- Activate your AWS access key via the AWS Console
- Validate the CloudFormation (CFn) template file
  - This is a different template file than the one used in a previous section
```
aws cloudformation validate-template --template-body file://template.yaml
```
- Deploy the Lambda
```
sam deploy --guided
```

## Undeploy With SAM

- Delete the CFn stack for the Lambda function
```
sam delete --stack-name cargo-lambda-prototype
```
- Optional: Delete the CFn stack for the SAM S3 bucket
  - The stack and S3 bucket might have been created during the guided deploy
  - You cannot delete the stack until you have deleted the S3 bucket
  - It is easiest to delete the S3 bucket from the AWS Console
    - Because you have to empty it before you can delete it
    - Including versioned objects
  - The S3 bucket will have a name like the following with a random suffix
    - aws-sam-cli-managed-default-samclisourcebucket-\[random]
```
aws cloudformation delete-stack --stack-name aws-sam-cli-managed-default
```
- Deactivate your AWS access key via the AWS Console

## History

- Initial release: 2023-12-01
