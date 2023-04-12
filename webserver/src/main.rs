use actix_web::{web, App, HttpResponse, HttpServer, Responder};


async fn get_index_page() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index_page))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
