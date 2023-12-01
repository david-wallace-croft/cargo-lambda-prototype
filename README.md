# Cargo Lambda Prototype

- Adapted from https://www.cargo-lambda.info/

## Tools Installation

- Install scoop
- Install cargo-lambda
  - scoop bucket add cargo-lambda https://github.com/cargo-lambda/scoop-cargo-lambda
  - scoop install cargo-lambda/cargo-lambda

## Usage

- cd cargo-lambda-prototype/
- cargo lambda watch
- http://localhost:9000/?name=World
- http://localhost:9000/lambda-url/cargo-lambda-prototype/?name=World

## Deploy

- cargo lambda build --release --arm64
- TODO
- cargo lambda deploy --enable-function-url --iam-role $env:IAMROLE

## History

- Initial release: 2023-12-01
