use warp::{Filter};

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello")
        .and(warp::path::param())
        .and(warp::get())
        .and(warp::header("user-agent"))
        .and(handle_rest);
    let hi = warp::path("hi").map(|| "Hello, World!");
    let ws = warp::path!("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            println!("New connection");
        });
    let routes = hello.or(hi);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn handle_rest(param: String, agent: String) -> String {
    format!("Hello {}, whose agent is {}", param, agent)
}