extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use actix::Actor;
use actix::Addr;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use code_server::code_server::CodeServer;
use code_session::CodeSession;
use dotenv::dotenv;
use log::{debug, info};
use rand::random;

mod models;
mod redis;
mod code_server;
mod code_session;
mod config;
mod stream_handler;

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<CodeServer>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (user_id, room_id) = path.into_inner();
    let code_session = CodeSession::new(random::<usize>(), srv.get_ref().clone(), room_id, user_id);
    ws::start(code_session, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    let code_server = CodeServer::new().start();

    info!("start server on : {}:{}", config::EXPOSED_IP, config::PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(code_server.clone()))
            .wrap(middleware::Logger::default())
            .route("/ws/user/{user_id}/room/{room_id}", web::get().to(websocket_handler))
            .service(
                web::resource("/hello").to(|| async { HttpResponse::Ok().body("Hello world!") }),
            )
    })
    .workers(2)
    .bind((config::EXPOSED_IP, config::PORT))?
    .run()
    .await
}

