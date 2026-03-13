use axum::{
    Json,
    routing::{get, post},
};
use axum::{Router, http::StatusCode, response::Html};
use serde::{Deserialize, Serialize};
use wstd::http::{Body, Error, Request, Response};

const INDEX: &'static str = include_str!("../html/index.html");
const ABOUT: &'static str = include_str!("../html/about.html");

#[wstd::http_server]
async fn main(request: Request<Body>) -> Result<Response<Body>, Error> {
    let service = Router::new()
        .route("/", get(handler))
        .route("/about", get(about_handler))
        .route("/user", post(create_user));
    wstd_axum::serve(request, service).await
}

async fn handler() -> Html<&'static str> {
    Html(INDEX)
}

async fn about_handler() -> Html<&'static str> {
    Html(ABOUT)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
