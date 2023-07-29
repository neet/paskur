mod y_history;

use actix_web::{get, http, web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Info {
    title: String,
}

#[get("/y-history/{title}")]
async fn y_history_controller(path: web::Path<Info>) -> Result<HttpResponse> {
    println!("Searching for {}", path.title);

    let result = y_history::search(&path.title).await;

    if result.is_err() {
        return Ok(HttpResponse::NotFound()
            .body("Could not find a video with that title. Please try again."));
    }

    let url = result.unwrap();

    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, url))
        .finish())
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_str = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let port = port_str.parse().expect("PORT must be a number");

    println!("Listening on {}", port);

    HttpServer::new(|| App::new().service(index).service(y_history_controller))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
