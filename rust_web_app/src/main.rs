// use actix_web::{web, App, HttpServer, HttpResponse, Responder};

// // Define a simple handler for the home route
// async fn home() -> impl Responder {
//     HttpResponse::Ok().body("Welcome to my Rust Web Application!")
// }

// // Define a handler for a hello route
// async fn hello(name: web::Path<String>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Hello, {}!", name))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Start the HTTP server
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(home)) // Home route
//             .route("/hello/{name}", web::get().to(hello)) // Hello route with a dynamic parameter
//     })
//     .bind("0.0.0.0:5000")? 
//     .run()
//     .await
// }

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::mysql::MySqlPoolOptions;
use serde::Serialize;
use dotenv::dotenv;
use std::env;

#[derive(Serialize)]
struct Post {
    id: i32,
    item: String,
}

async fn fetch_posts(db_pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let posts: Vec<Post> = match sqlx::query_as!(
        Post,
        r#"SELECT id, item FROM Post"#
    )
    .fetch_all(db_pool.get_ref())
    .await
    {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch posts"),
    };

    let list_items: String = posts
        .into_iter()
        .map(|post| format!("<li>{}</li>", post.item))
        .collect();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<ul>{}</ul>", list_items))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/", web::get().to(fetch_posts))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}


