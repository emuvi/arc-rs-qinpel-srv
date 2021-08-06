use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

fn init_run_folder() {
    println!("init run folder..........");
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("QinpelSrv : 0.1.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Qinpel Server booting...");
    init_run_folder();
    println!("Qinpel Server starting...");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/run", "./run").index_file("index.html"))
    })
    .bind("0.0.0.0:5490")?
    .run()
    .await
}
