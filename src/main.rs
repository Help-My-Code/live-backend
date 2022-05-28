use actix::Addr;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web_actors::ws;
use code_session::{CodeServer, CodeSession};
use rand::random;

mod code_session;

async fn websocket_handler(req: HttpRequest, stream: web::Payload,
    srv: web::Data<Addr<CodeServer>>) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let code_session = CodeSession::new(random::<usize>(), srv.get_ref().clone());
    ws::start(code_session, &req, stream)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/ws", web::get().to(websocket_handler))
            .service(web::resource("/").to(|| async {
                HttpResponse::Ok().body("Hello world!")
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}