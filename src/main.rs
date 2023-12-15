use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::request::RequestContext;
use lambda_http::{
  run, service_fn, Body, Error, Request, RequestExt, Response,
};

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
  // println!("request:\n{:#?}", request);
  let request_str = format!("{:#?}", request);
  let request_context: RequestContext = request.request_context();
  let request_context_str = format!("{:#?}", request_context);
  let query_string_parameters: QueryMap = request.query_string_parameters();
  let query_string_parameters_str = format!("{:#?}", query_string_parameters);
  let name = request
    .query_string_parameters_ref()
    .and_then(|params| params.first("name"))
    .unwrap_or("World");
  let message_str = format!("Hello, {}!", name);
  let response_body_str = format!(
    "\n{}\n\n{}\n\n{}\n\n{}",
    request_str, request_context_str, query_string_parameters_str, message_str
  );
  let response_body = Body::from(response_body_str);
  let response = Response::builder()
    .status(200)
    .header("content-type", "text/plain")
    .body(response_body)
    .map_err(Box::new)?;
  Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .with_target(false)
    .without_time()
    .init();
  run(service_fn(function_handler)).await
}
