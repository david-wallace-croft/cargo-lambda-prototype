use lambda_http::request::RequestContext;
use lambda_http::request::RequestContext::ApiGatewayV1;
use lambda_http::request::RequestContext::ApiGatewayV2;
use lambda_http::{
  run, service_fn, Body, Error, Request, RequestExt, Response,
};

/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  println!("event:\n{:#?}", event);
  let request_context: RequestContext = event.request_context();
  println!("request_context:\n{:#?}", request_context);
  // TODO: use a match
  if let ApiGatewayV1(api_gateway_v1_http_request) = request_context {
    println!(
      "api_gateway_v1_http_request:\n{:#?}",
      api_gateway_v1_http_request
    );
  } else if let ApiGatewayV2(api_gateway_v2_http_request) = request_context {
    println!(
      "api_gateway_v2_http_request:\n{:#?}",
      api_gateway_v2_http_request
    );
  }

  // Extract some useful information from the request
  let who = event
    .query_string_parameters_ref()
    .and_then(|params| params.first("name"))
    .unwrap_or("world");
  let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

  // Return something that implements IntoResponse.
  // It will be serialized to the right response event automatically by the runtime
  let resp = Response::builder()
    .status(200)
    .header("content-type", "text/html")
    .body(message.into())
    .map_err(Box::new)?;
  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

  run(service_fn(function_handler)).await
}
