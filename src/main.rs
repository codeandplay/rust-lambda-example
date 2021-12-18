#[macro_use]
extern crate log;
use simple_logger::SimpleLogger;

use lambda_http::{handler, lambda_runtime, Context, IntoResponse, Request, RequestExt};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .env()
        .with_utc_timestamps()
        .init()
        .unwrap();
    lambda_runtime::run(handler(hello)).await?;
    Ok(())
}

async fn hello(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!({
        "message":
            format!(
                "hello {}",
                req.query_string_parameters()
                    .get("name")
                    .unwrap_or_else(|| "stranger")
            )
    }))
}
