use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn test() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("http_server", &local);
    HttpServer::new(move || {
        App::new()
            .route("/test", web::post().to(test))
    })
        .bind("127.0.0.1:8080")
        .expect("Cannot bind to address:port")
        .run();

    tokio::spawn(sys).await;
}
