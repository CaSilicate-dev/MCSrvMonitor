use chrono::Utc;
use rocket::futures::future::join_all;
use rusqlite::Connection;
use rust_mc_status::{McClient, ServerData, ServerEdition};
use serde::Deserialize;
use std::fs;
use tokio::time::{Duration, sleep};

//#[derive(Deserialize)]
/*struct Config {
    database_filename: String,
    interval_sec: u32,
    server_addr: String,
}
struct SingleServer {
    name: String,
    addr: String,
}*/
#[derive(Debug, Deserialize)]
struct Config {
    //port: u16,
    //length: u32,
    backend: BackendConfig,
    //frontend: FrontendConfig,
    servers: Vec<SingleServerConfig>,
}
#[derive(Debug, Deserialize)]
struct BackendConfig {
    dbfile: String,
    interval: u32,
}
#[derive(Debug, Deserialize, Clone)]
struct SingleServerConfig {
    name: String,
    addr: String,
}
#[derive(Debug, Default)]
struct ResultData {
    latencies: Vec<i32>,
    players: Vec<i32>,
    playerlists: Vec<Vec<String>>,
}
fn load_config() -> Config {
    //let contents = fs::read_to_string("config.yaml").unwrap();
    let configfile = fs::read_to_string("config.json");
    let contents = match configfile {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open essential config: {} ", e);

            std::process::exit(1);
        }
    };
    let cont = serde_json::from_str(&contents);
    match cont {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to parse essential config: {}", e);
            std::process::exit(1);
        }
    }
}
fn record(ts: i64, dbfile: &String, rd: ResultData, serverlist: &[SingleServerConfig]) {
    let connection = match Connection::open(dbfile) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open essential database: {}", e);
            std::process::exit(1);
        }
    };
    for (i, _) in serverlist.iter().enumerate() {
        let _ = connection.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                timestamp INTEGER PRIMARY KEY,
                latency INTEGER NOT NULL,
                players INTEGER NOT NULL,
                playerlist TEXT NOT NULL
                )",
                serverlist[i].name.as_str()
            )
            .as_str(),
            (),
        );

        let _ = connection.execute(
            format!(
                "INSERT INTO {} (timestamp, latency, players, playerlist)
                VALUES (?1,?2,?3,?4)",
                serverlist[i].name.as_str()
            )
            .as_str(),
            (
                ts,
                rd.latencies[i],
                rd.players[i],
                match serde_json::to_string(&rd.playerlists[i]) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                },
            ),
        );
    }
}
fn get_time() -> i64 {
    Utc::now().timestamp()
}
async fn get_data(client: &McClient, servers: &Vec<SingleServerConfig>) -> ResultData {
    let mut data = ResultData::default();
    let mut server_futures = Vec::new();
    for server in servers {
        let client = client.clone();
        let addr = server.addr.clone();
        let cf = async move {
            let mut players = Vec::new();
            println!("Sending request to {}", addr);
            let (latency, player_count) = match client.ping(&addr, ServerEdition::Java).await {
                Ok(status) => {
                    let latency = status.latency as i32;
                    match status.data {
                        ServerData::Java(status) => {
                            let player_count = status.players.online as i32;
                            match status.players.sample {
                                Some(pl) => {
                                    for i in pl.iter() {
                                        players.push(i.name.clone());
                                    }
                                    (latency, player_count)
                                }
                                None => (latency, 0),
                            }
                        }
                        ServerData::Bedrock(_) => (latency, 0),
                    }
                }
                Err(_) => (-1, -1),
            };
            println!("Get the result of {}", addr);
            (latency, player_count, players)
        };
        server_futures.push(cf);
    }
    let resultdataf = join_all(server_futures).await;
    for (latency, player_count, player_list) in resultdataf {
        data.latencies.push(latency);
        data.players.push(player_count);
        data.playerlists.push(player_list);
    }
    //return (latency, players, serde_json::to_string(&pll).unwrap());
    return data;
}

pub async fn run() {
    let conf = load_config();
    for server in &conf.servers {
        if server.name.chars().all(|c| c.is_ascii_lowercase()) {
        } else {
            eprint!("Invalid server name: {}", server.name);
            std::process::exit(1);
        }
    }
    let client = McClient::new();
    //print!("{:?}", conf);
    loop {
        let ct = get_time();
        if ct % (conf.backend.interval as i64) == 0 {
            println!("{:?}", "run");
            let rd = get_data(&client, &conf.servers).await;
            println!("{:?}", rd);
            record(ct, &conf.backend.dbfile, rd, &conf.servers);
            sleep(Duration::from_millis(1000)).await;
        }
        println!("{:?}", ct);
        sleep(Duration::from_millis(500)).await;
    }
}
