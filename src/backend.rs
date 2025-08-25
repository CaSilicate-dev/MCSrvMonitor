use chrono::Utc;
use rusqlite::Connection;
use rust_mc_status::{McClient, ServerData, ServerEdition};
use serde::Deserialize;
use std::fs;
use tokio::time::{Duration, sleep};

#[derive(Deserialize)]
struct Config {
    database_filename: String,
    interval_sec: u32,
    server_addr: String,
}
fn load_config() -> Config {
    //let contents = fs::read_to_string("config.yaml").unwrap();
    let configfile = fs::read_to_string("config.yaml");
    let contents;
    match configfile {
        Ok(r) => {
            contents = r;
        }
        Err(e) => {
            eprint!("Failed to open essential config: {} \n", e);
            std::process::exit(1);
        }
    }
    let cont = serde_yaml::from_str(&contents);
    let config: Config;
    match cont {
        Ok(r) => {
            config = r;
        }
        Err(e) => {
            eprint!("Failed to open essential config: {}\n", e);
            std::process::exit(1);
        }
    }

    return config;
}
fn record(ts: i64, lc: i32, pl: i32, filename: String) {
    let connection = match Connection::open(filename) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Failed to open database: {}\n", e);
            std::process::exit(1);
        }
    };
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS mcserver (
            timestamp INTEGER PRIMARY KEY,
            latency INTEGER NOT NULL,
            players INTEGER NOT NULL
        )",
            [],
        )
        .expect("Failed to create table");

    let _ = connection.execute(
        format!(
            "INSERT INTO mcserver (timestamp, latency, players)
            VALUES ({},{},{})",
            ts, lc, pl
        )
        .as_str(),
        (),
    );
}
fn get_time() -> i64 {
    let ctimestamp = Utc::now().timestamp();
    return ctimestamp;
}
async fn get_data(client: &McClient, addr: String) -> (i32, i32) {
    let status = client.ping(addr.as_str(), ServerEdition::Java).await;
    let latency;
    let players;
    match status {
        Ok(status) => {
            latency = status.latency as i32;
            let data = status.data;
            match data {
                ServerData::Java(status) => {
                    players = status.players.online as i32;
                }
                ServerData::Bedrock(_) => {
                    players = -1;
                }
            }
        }
        Err(_) => {
            latency = -1;
            players = -1;
        }
    }
    return (latency, players);
}
#[tokio::main]
pub async fn run() {
    let conf = load_config();
    let client = McClient::new();
    loop {
        let ct = get_time();
        if ct % (conf.interval_sec as i64) == 0 {
            let (l, p) = get_data(&client, conf.server_addr.clone()).await;
            record(ct, l, p, conf.database_filename.clone());
            sleep(Duration::from_millis(1000)).await;
        }
        sleep(Duration::from_millis(500)).await;
    }
}
