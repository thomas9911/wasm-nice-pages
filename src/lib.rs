use axum::{Router, response::Html, routing::get};
use wstd::http::{Body, Error, Request, Response};

const INDEX: &'static str = include_str!("../html/index.html");
const ABOUT: &'static str = include_str!("../html/about.html");

#[wstd::http_server]
async fn main(request: Request<Body>) -> Result<Response<Body>, Error> {
    let service = Router::new()
        .route("/", get(handler))
        .route("/about", get(about_handler));
    wstd_axum::serve(request, service).await
}

async fn handler() -> Html<&'static str> {
    Html(INDEX)
}

async fn about_handler() -> Html<&'static str> {
    Html(ABOUT)
}
