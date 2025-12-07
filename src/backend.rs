use chrono::Utc;
use rocket::futures::future::join_all;
use rusqlite::Connection;
use rust_mc_status::{McClient, ServerData, ServerEdition};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::time::{Duration, sleep};

macro_rules! file_base {
    () => { "./data/" };
}

const CONFIG_FILE: &str = &concat!(file_base!(), "config.json");
const DB_FILE: &str = &concat!(file_base!(), "history.db");

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
#[derive(Serialize, Deserialize, Clone)]
struct ConfigFile {
    addr: String,
    port: u16,
    length: u32,
    backend: BackendConfig,
    servers: Vec<SingleServerConfig>,
}
impl Default for ConfigFile {
    fn default() -> Self {
        Self { addr: "127.0.0.1".to_string(), port: 9010, length: 100, backend: BackendConfig::default(), servers: vec![SingleServerConfig::default()] }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct BackendConfig {
    //#[allow(unused)]
    interval: u32,
}
impl Default for BackendConfig {
    fn default() -> Self {
        Self { interval: 1 }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct SingleServerConfig {
    name: String,
    label: String,
    addr: String,
    #[serde(rename = "type")]
    stype: i8,
}
impl Default for SingleServerConfig {
    fn default() -> Self {
        Self { name: "server".to_string(), label: "Server1".to_string(), addr: "127.0.0.1".to_string(), stype: 1 }
    }
}
#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct SingleServerData {
    timestamp: String,
    #[serde(rename = "type")]
    stype: i8,
    latency: i32,
    player: i32,
    playerlist: String,
}
impl Default for SingleServerData {
    fn default() -> Self {
        SingleServerData {
            timestamp: "None".to_string(),
            stype: -1,
            latency: -1,
            player: -1,
            playerlist: "".to_string(),
        }
    }
}
#[derive(Debug, Default)]
struct ResultData {
    latencies: Vec<i32>,
    players: Vec<i32>,
    playerlists: Vec<Vec<String>>,
}
fn load_config_raw() -> Result<ConfigFile, Box<dyn std::error::Error>> {
    let configfile = fs::read_to_string(CONFIG_FILE)?;
    let cont = serde_json::from_str(&configfile)?;
    Ok(cont)
}
fn load_config() -> ConfigFile {
    let rc = load_config_raw();
    match rc {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to load configfile: {}", e);
            println!("");
            let default_config = ConfigFile::default();
            write_config(default_config.clone());
            default_config
        }
    }
}
fn write_config_raw(configjson: &str) -> std::io::Result<()> {
    fs::write(CONFIG_FILE, configjson)?;
    Ok(())
}
fn write_config(config: ConfigFile) {
    let json_string = match serde_json::to_string_pretty(&config) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to write config file: {}", e);
            return;
        }
    };
    write_config_raw(json_string.as_str()).unwrap_or_else(|e| {
        eprintln!("Failed to write config file: {}", e);
    });
}

fn record(ts: i64, dbfile: &String, rd: ResultData, serverlist: &[SingleServerConfig]) {
    let connection = match Connection::open(dbfile) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open essential database: {}", e);
            std::process::exit(1);
        }
    };
    for (i, c) in serverlist.iter().enumerate() {
        let _ = connection.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                timestamp INTEGER PRIMARY KEY,
                type INTEGER NOT NULL,
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
                "INSERT INTO {} (timestamp, type, latency, players, playerlist)
                VALUES (?1,?2,?3,?4,?5)",
                serverlist[i].name.as_str()
            )
            .as_str(),
            (
                ts,
                c.stype,
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
        let stype = server.stype.clone();
        //eprint!("\n\n\n{}\n\n\n",stype);
        let cf = async move {
            let mut players = Vec::new();
            let latency;
            let player_count;
            println!("Sending request to {}", addr);
            if stype == 1 {
                (latency, player_count) = match client.ping(&addr, ServerEdition::Java).await {
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
                            ServerData::Bedrock(_) =>
                            /*(latency, 0)*/
                            {
                                (-1, -1)
                            }
                        }
                    }
                    Err(_) => (-1, -1),
                };
            } else if stype == 0 {
                (latency, player_count) = match client.ping(&addr, ServerEdition::Bedrock).await {
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
                            ServerData::Bedrock(status) =>
                            /*(latency, 0)*/
                            {
                                (latency, status.online_players.parse().unwrap())
                            }
                        }
                    }
                    Err(_) => (-1, -1),
                };
            } else {
                (latency, player_count) = (-1, -1)
            }
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
            record(ct, &DB_FILE.to_string(), rd, &conf.servers);
            sleep(Duration::from_millis(1000)).await;
        }
        println!("{:?}", ct);
        sleep(Duration::from_millis(500)).await;
    }
}
