use actix_web::{get, http::header::ContentType, web, App, Error, HttpResponse, HttpServer};
use reqwest::StatusCode;
use serde_json;

mod actions;
mod types;

#[get("/buses")]
async fn read_all() -> Result<HttpResponse, Error> {
    let buses = actions::get_buses().await.unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(format!("{}", serde_json::to_string(&buses).unwrap())))
}

#[get("/buses/{number}")]
async fn read(path: web::Path<(String,)>) -> Result<HttpResponse, Error> {
    let bus = actions::get_bus(path.into_inner().0).await.unwrap();

    match bus {
        Some(bus_found) => Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::json())
            .body(format!("{}", serde_json::to_string(&bus_found).unwrap()))),
        None => Ok(HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type(ContentType::json())
            .body("")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(read_all).service(read))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
