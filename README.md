# Cargo Lambda Prototype

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-prototype/blob/main/LICENSE.txt

- Deploy a Rust programming language serverless Function-as-a-Service (FaaS)
- Uses Cargo-Lambda, Amazon Web Services (AWS) Lambda, and AWS CloudFormation
- Adapted from https://www.cargo-lambda.info/

## CLI Installation

- Install the AWS Command-Line Interface (CLI)
  - https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
- Install the AWS Serverless Application Model (SAM) CLI
  - https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html
  - If you are using Git Bash for Windows
    - You will need to enter "sam.cmd" instead of "sam" to run the SAM CLI

## Cargo-Lambda Installation

- https://www.cargo-lambda.info/guide/installation.html
- Installation instructions for the Windows operating system
  - Install scoop
    - https://scoop.sh/
  - Install cargo-lambda
```
scoop bucket add cargo-lambda https://github.com/cargo-lambda/scoop-cargo-lambda
scoop install cargo-lambda/cargo-lambda
```

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
```
cargo lambda build --release
```

## Deploy Using Cargo-Lambda

- Activate your AWS access key via the AWS Console
- Deploy the Lambda to the cloud
  - This will automatically generate a role for you
  - Note the generated URL
```
cargo lambda deploy --enable-function-url
```
- Test
  - Example URL: https://\[abc123].lambda-url.\[region].on.aws/?name=World

## Undeploy Using Cargo-Lambda

- It appears that Cargo-Lambda does not currently have an undeploy option
  - So you will need to undeploy using the AWS CLI
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

## Deploy Using a CloudFormation Template for the Role

- Activate your AWS access key via the AWS Console
- Validate the CloudFormation (CFn) template file
  - Note that the file is named template-role.yaml instead of template.yaml
```
aws cloudformation validate-template --template-body file://template-role.yaml
```
- Create a CloudFormation (CFn) stack that defines a Lambda execution role
```
aws cloudformation create-stack \
  --capabilities CAPABILITY_NAMED_IAM \
  --stack-name cargo-lambda-prototype \
  --template-body file://template-role.yaml
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

## Undeploy Using a CloudFormation Template for the Role

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

## Deploy Using the SAM CLI

- Activate your AWS access key via the AWS Console
- Validate the CloudFormation (CFn) template file
  - This is a different template file than the one used in a previous section
```
aws cloudformation validate-template --template-body file://template.yaml
```
- Deploy the Lambda
  - Note the output URL
```
sam deploy --guided
```
- Test by using the output URL
  - Example: https://a1b2c3.execute-api.us-east-1.amazonaws.com/?name=World

## Undeploy Using the SAM CLI

- Delete the CFn stack for the Lambda function
```
sam delete
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

## Deploy Using Authentication

- This example uses OpenID Connect (OIDC) on top of OAuth 2.0
- Validate the CloudFormation (CFn) template file
  - This is a different template file than the one used in a previous section
```
aws cloudformation validate-template --template-body file://template-auth.yaml
```
- Deploy the Lambda
  - Note the outputs CargoLambdaHttpApiUrl and CargoLambdaSignupUrl
```
sam deploy -t template-auth.yaml --guided
```
- Test by using the output URL
  - You should get "Unauthorized" as the response
  - Example: https://a1b2c3.execute-api.us-east-1.amazonaws.com/?name=World
- Open your browser
- Open the browser developer console (F12) and start monitoring Network
- Enter the CargoLambdaSignupUrl in your browser
- Click on "Sign up" to create a new user account
- Log in with the new user account
- Note in the developer console Network monitoring the 302 redirect location
  - There should be a code parameter with a UUID value
- Exchange the code for tokens
```
curl --location --request POST \
  https://a1b2c3-d4e5f6.auth.us-east-1.amazoncognito.com/oauth2/token \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'client_id=d4e5f6' \
  --data-urlencode 'code=g7h8i9' \
  --data-urlencode 'grant_type=authorization_code' \
  --data-urlencode 'scope=openid' \
  --data-urlencode 'redirect_uri=http://localhost:8080'
```
- Extract the access_token from the response
  - It might be between the id_token and the refresh_token
- Use the access_token to get the user information
```
curl -X GET \
  https://a1b2c3-d4e5f6.auth.us-east-1.amazoncognito.com/oauth2/userInfo \
  -H 'Authorization: Bearer <access_token>'
```
- Use the access_token to access the lambda function
  - You will not get "Unauthorized" this time
```
curl -X GET \
  https://a1b2c3.execute-api.us-east-1.amazonaws.com/ \
  -H 'Authorization: Bearer <access_token>'
```

## Undeploy Using Authentication

- Delete the CFn stack for the Lambda function
```
sam delete
```
- Deactivate your AWS access key via the AWS Console

## History

- Initial release: 2023-12-01
