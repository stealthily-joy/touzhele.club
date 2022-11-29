use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use touzhele::db::{create_post as db_create_post, list_post as db_list_post};
use actix_files as fs;

#[post("/create_post")]
async fn create_post(text: String) -> impl Responder {
    let post = db_create_post(&text);
    HttpResponse::Ok().body(post.id.to_string())
}

#[get("/list_post")]
async fn list_post() -> impl Responder {
    let posts = db_list_post();
    HttpResponse::Ok().body(format!("{:#?}", posts))
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_post)
            .service(list_post)
            .service(health_check)
            .service(fs::Files::new("/", "./web/dist/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}