use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Qinpel Server starting...");
    HttpServer::new(|| App::new().service(ping))
        .bind("0.0.0.0:5490")?
        .run()
        .await
}
