use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

const PROJECT_NAME: &str = "Rust-Server-101";

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome to {}", PROJECT_NAME))
}

#[derive(Deserialize)]
struct GreetQuery {
    name: String,
}

#[get("/greetme")]
async fn greet_with_query(info: web::Query<GreetQuery>) -> impl Responder {
    let name = &info.name;

    HttpResponse::Ok().body(format!("Hello {name}! Nice to meet you."))
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    name: String,
}

#[post("/createuser")]
async fn create_user(user: web::Json<CreateUser>) -> impl Responder {
    let response = json!({
        "messgae":"Post operation complete",
        "data":Json(user.into_inner()),
    });

    HttpResponse::Created().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(greet_with_query)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
