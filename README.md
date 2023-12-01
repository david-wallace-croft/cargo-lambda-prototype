# Cargo Lambda Prototype

- Adapted from https://www.cargo-lambda.info/

## Tools Installation for Windows

- Install scoop
  - https://scoop.sh/
- Install cargo-lambda
  - scoop bucket add cargo-lambda https://github.com/cargo-lambda/scoop-cargo-lambda
  - scoop install cargo-lambda/cargo-lambda

## Usage

- cd cargo-lambda-prototype/
- cargo lambda watch
- http://localhost:9000/?name=World
- http://localhost:9000/lambda-url/cargo-lambda-prototype/?name=World
- Open a new terminal
- cargo lambda -v invoke --data-example apigw-request cargo-lambda-prototype --output-format json
- Note the DEBUG output showing the location of the cached example input file
- Edit the cached example input file to change the name
  - Look for the name property under multiValueQueryStringParameters
- Run the cargo lambda invoke command again
  - The output should show the name that you changed to in the example input file

## Deploy

- Build a release for the ARM64 architecture
```
cargo lambda build --release --arm64
```
- Set up your AWS credentials
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
  - Note the name of the role that was automatically generated for you
  - You will need it in a following step to delete the role
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
  --role-name cargo-lambda-role-[UUID]
  --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
```
- Delete the role
```
aws iam delete-role --role-name cargo-lambda-role-[UUID]
```
- Optional: Deactivate your AWS access key until you need it again

## History

- Initial release: 2023-12-01
