use actix::Addr;
use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web::web::get;
use actix_web_actors::ws;

mod code_session;

async fn websocket_handler(req: HttpRequest, stream: web::Payload,
    srv: web::Data<Addr<code_session::CodeSession>>) -> HttpResponse {
    println!("{:?}", req);
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/ws". web::get().to(websocket_handler))
            .service(web::resource("/").to(|| async {
                HttpResponse::Ok().body("Hello world!")
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}