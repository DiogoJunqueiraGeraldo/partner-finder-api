use std::collections::HashMap;

use serde::Serialize;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Serialize)]
struct Location {
    lat: f32,
    lng: f32
}

#[derive(Serialize)]
struct Partner {
    name: String,
    location: Location,
}

#[derive(Serialize)]
struct SuccessMessage {
    search: Location,
    partner: Partner
}


async fn find_partner(query_params: web::Query<HashMap<String, f32>>) -> impl Responder {
    let try_lat = query_params.get("lat");
    let try_lng = query_params.get("lng");

    if try_lat.and(try_lng).is_some() {
        let lat = *try_lat.unwrap();
        let lng = *try_lng.unwrap();

        HttpResponse::Ok().json(SuccessMessage {
            search: Location { lat , lng },
            partner: Partner {
                name: String::from("Default"),
                location: Location { lat , lng }
            }
        })
    } else {
        HttpResponse::BadRequest().json(ErrorMessage {
            message: String::from("To find a partner lat and lng query params is required")
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/partners", web::get().to(find_partner))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}