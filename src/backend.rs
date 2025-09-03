use chrono::Utc;
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
struct FrontendConfig {
    //addr: String,
    //port: u16,
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
    length: u32,
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
    for server in servers {
        println!("{:?}", server.addr.as_str());
        let status = client.ping(server.addr.as_str(), ServerEdition::Java).await;
        //println!("{:?}",status);
        let mut players: Vec<String> = Vec::new();
        match status {
            Ok(status) => {
                data.latencies.push(status.latency as i32);
                let sdata = status.data;
                match sdata {
                    ServerData::Java(status) => {
                        data.players.push(status.players.online as i32);
                        if let Some(pdata) = status.players.sample {
                            for i in pdata {
                                println!("{:?}", i.name);
                                players.push(i.name);
                            }
                        }
                    }
                    ServerData::Bedrock(_) => {
                        data.players.push(-1);
                    }
                }
                data.playerlists.push(players);
            }
            Err(_) => {
                data.playerlists.push(players);
                data.latencies.push(-1);
                data.players.push(-1);
            }
        }
        data.length += 1;
    }
    data
    //return (latency, players, serde_json::to_string(&pll).unwrap());
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
