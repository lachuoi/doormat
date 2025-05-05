use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
async fn handle_root(req: Request) -> Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Welcome")
        .build())
}
