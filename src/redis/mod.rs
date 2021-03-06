extern crate redis;
use std::env;

use crate::models::delta::Delta;

pub fn add_deltas(deltas: &Vec<Delta>, room_id: &String) -> redis::RedisResult<()> {
    let redis_url = env::var("REDIS_URL").unwrap_or(String::from("redis:127.0.0.1:6379"));
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    for delta in deltas {
        let delta = serde_json::to_string(&delta).unwrap();
        redis::cmd("RPUSH")
            .arg(format!("{}_{}", room_id, "delta"))
            .arg(delta)
            .execute(&mut con);
    }
    Ok(())
}

pub fn get_current_context_for_room(room_id: &String) -> redis::RedisResult<Vec<Delta>> {
    let redis_url = env::var("REDIS_URL").unwrap_or(String::from("redis:127.0.0.1:6379"));
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    let deltas: redis::Iter<String> = redis::cmd("LRANGE")
        .arg(format!("{}_{}", room_id, "delta"))
        .arg::<i8>(0)
        .arg::<i8>(-1)
        .clone()
        .iter(&mut con)
        .unwrap();

    let deltas = deltas.map(|delta| serde_json::from_str(&delta).expect("cannot deserialize delta")).collect();
    Ok(deltas)
}
