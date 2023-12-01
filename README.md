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
- Note DEBUG output showing the location of the cached example input file
- Edit the cached example input file to change the name
  - Look for the name property under multiValueQueryStringParameters
- Run the cargo lambda invoke command again
  - The output should show the name that you changed in the example input file

## Deploy

- cargo lambda build --release --arm64
- TODO
- cargo lambda deploy --enable-function-url --iam-role $env:IAMROLE

## History

- Initial release: 2023-12-01
