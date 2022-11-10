#[cfg(feature = "fixed-template")]
use askama::Template;

#[cfg(feature = "hot-template")]
use axum::response::Html;
#[cfg(feature = "hot-template")]
use tera::{Context, Tera};

use axum::{
    http::header,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::fs;

#[cfg(feature = "fixed-template")]
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}
#[cfg(feature = "fixed-template")]
async fn index() -> Result<impl IntoResponse, ()> {
    let template = HelloTemplate { name: "world" };
    Ok(template)
}

#[cfg(feature = "hot-template")]
async fn index() -> Result<impl IntoResponse, ()> {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    context.insert("name", "world");
    let html = tera.render("hello.html", &context).unwrap();
    Ok(Html(html))
}

async fn stylesheet() -> Result<impl IntoResponse, ()> {
    let stylesheet = fs::read_to_string("styles/index.css").await.unwrap();
    let resp = Response::builder()
        .header(header::CONTENT_TYPE, "text/css")
        .body(stylesheet)
        .unwrap();
    Ok(resp)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/index.css", get(stylesheet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
