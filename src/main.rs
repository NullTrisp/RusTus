use actix_web::{get, http::header::ContentType, web, App, Error, HttpResponse, HttpServer};
use reqwest::StatusCode;

mod actions;
mod types;

#[get("/buses")]
async fn read_all_buses() -> Result<HttpResponse, Error> {
    let buses = actions::get_buses().await.unwrap();

    Ok(HttpResponse::build(StatusCode::OK).json(&buses))
}

#[get("/buses/{bus_number}")]
async fn read_bus(path: web::Path<(String,)>) -> Result<HttpResponse, Error> {
    let bus = actions::get_bus(path.into_inner().0).await.unwrap();

    match bus {
        Some(bus_found) => Ok(HttpResponse::build(StatusCode::OK).json(&bus_found)),
        None => Ok(HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type(ContentType::json())
            .body("")),
    }
}

#[get("/stops")]
async fn read_all_stops(params: web::Query<types::Offest>) -> Result<HttpResponse, Error> {
    match actions::get_stops(params.into_inner()).await {
        Ok(buses) => Ok(HttpResponse::build(StatusCode::OK).json(&buses)),
        Err(err) => Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .body(format!("{}", err))),
    }
}

#[get("/stops/{stop_number}")]
async fn read_stop(path: web::Path<(String,)>) -> Result<HttpResponse, Error> {
    match actions::get_stop(path.into_inner().0).await {
        Ok(bus_found) => match bus_found {
            Some(bus) => Ok(HttpResponse::build(StatusCode::OK).json(bus)),
            None => Ok(HttpResponse::build(StatusCode::NOT_FOUND)
                .content_type(ContentType::json())
                .body("")),
        },
        Err(err) => Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .body(format!("{}", err))),
    }
}

#[get("/estimations/{stop_number}")]
async fn read_estimation(path: web::Path<(String,)>) -> Result<HttpResponse, Error> {
    match actions::get_stop(path.into_inner().0).await {
        Ok(stop_found) => match stop_found {
            Some(stop) => Ok(HttpResponse::build(StatusCode::OK)
                .json(actions::get_estimation(stop).await.unwrap())),
            None => Ok(HttpResponse::build(StatusCode::NOT_FOUND)
                .content_type(ContentType::json())
                .body("")),
        },
        Err(err) => Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .body(format!("{}", err))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(read_all_buses)
            .service(read_bus)
            .service(read_all_stops)
            .service(read_stop)
            .service(read_estimation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
