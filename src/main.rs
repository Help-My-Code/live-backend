use actix::Addr;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web_actors::ws;
use actix::Actor;
use code_server::CodeServer;
use code_session::CodeSession;
use rand::random;

mod event;
mod code_session;
mod code_server;
mod config;
mod program_dto;

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<CodeServer>>,
    path: web::Path<String>
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let room_name = path.into_inner();
    println!("{:?}", room_name);
    let code_session = CodeSession::new(random::<usize>(), srv.get_ref().clone(), room_name, None);
    ws::start(code_session, &req, stream)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let code_server = CodeServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(code_server.clone()))
            .wrap(middleware::Logger::default())
            .route("/ws/{room_id}", web::get().to(websocket_handler))
            .service(web::resource("/hello").to(|| async {
                HttpResponse::Ok().body("Hello world!")
            }))
    })
    .workers(2)
    .bind((config::EXPOSED_IP, config::PORT))?
    .run()
    .await
}
