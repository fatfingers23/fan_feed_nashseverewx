use actix_files::{Files};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(||
        App::new()
            .service(Files::new("/", "./static_files")
                .index_file("index.html"))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}