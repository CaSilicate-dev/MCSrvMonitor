use chrono::{DateTime, Local, TimeZone};
use regex::Regex;
use rocket::{config::Config, serde::json::Json};
use rocket_cors::CorsOptions;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
mod backend;

macro_rules! file_base {
    () => { "./data/" };
}

const CONFIG_FILE: &str = &concat!(file_base!(), "config.json");
const DB_FILE: &str = &concat!(file_base!(), "history.db");

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
        Self { addr: "0.0.0.0".to_string(), port: 9010, length: 100, backend: BackendConfig::default(), servers: vec![SingleServerConfig::default()] }
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
#[derive(Serialize, Debug, Default)]
struct ResponseData {
    label: String,
    data: Vec<SingleServerData>,
}
#[derive(Serialize, Debug, Default)]
struct ResponseList {
    typelist: Vec<i8>,
    namelist: Vec<String>,
    labellist: Vec<String>,
}

#[derive(Serialize, Debug, Default)]
struct ResponseRawServerData {
    data: String,
}

#[macro_use]
extern crate rocket;

fn is_valid_string(s: &str) -> Result<bool, regex::Error> {
    let aaa: Regex = Regex::new(r"^[A-Za-z0-9_]+$")?;
    return Ok(aaa.is_match(s));
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
fn get_record(
    servername: String,
    filename: String,
    servers: Vec<SingleServerConfig>,
    length: u32,
) -> Result<ResponseData, String> {
    let mut resp = ResponseData::default();

    let conn = match Connection::open(filename) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Failed to open database {}", e));
        }
    };
    let server = match servers.iter().find(|s| s.name == servername) {
        Some(r) => r,
        _ => {
            return Err(format!("Invalid server name: {}", servername));
        }
    };
    resp.label = server.label.clone();
    let mut stmt = match conn.prepare(&format!(
        "SELECT * FROM {} ORDER BY timestamp DESC LIMIT {}",
        server.name, length,
    )) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Failed to read database {}", e));
        }
    };
    let rows = match stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i8>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, String>(4)?,
        ))
    }) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Invalid db data {}", e));
        }
    };

    for row in rows {
        let mut single_resp = SingleServerData::default();
        let (ctimestamp, stype, clatency, cplayer, cplayerlist) = row.unwrap();
        #[allow(deprecated)]
        let local_time: DateTime<Local> = Local.timestamp(ctimestamp, 0);

        single_resp.timestamp = local_time.to_string();
        single_resp.stype = stype;
        single_resp.latency = clatency;
        single_resp.player = cplayer;
        single_resp.playerlist = cplayerlist.to_string();
        resp.data.push(single_resp);
    }
    Ok(resp)
}

#[get("/api/servers/<servername_in>")]
fn index_api_servers_servername(servername_in: &str) -> Json<ResponseData> {
    let servername = servername_in.to_string();
    let conf = load_config();
    match get_record(servername, DB_FILE.to_string(), conf.servers, conf.length) {
        Ok(r) => Json(r),
        Err(e) => Json(ResponseData {
            label: "Error".to_string(),
            data: vec![SingleServerData {
                playerlist: e,
                ..Default::default()
            }],
        }),
    }
}
#[get("/api/serverod/<servername_in>")]
fn index_api_serverod_servername(servername_in: &str) -> Json<ResponseData> {
    let servername = servername_in.to_string();
    let conf = load_config();
    match get_record(servername, DB_FILE.to_string(), conf.servers, 1) {
        Ok(r) => Json(r),
        Err(e) => Json(ResponseData {
            label: "Error".to_string(),
            data: vec![SingleServerData {
                playerlist: e,
                ..Default::default()
            }],
        }),
    }
}
#[get("/api/list")]
fn index_api_list() -> Json<ResponseList> {
    let conf = load_config();
    let mut resp = ResponseList::default();
    for server in conf.servers {
        resp.typelist.push(server.stype);
        resp.namelist.push(server.name);
        resp.labellist.push(server.label);
    }
    Json(resp)
}
#[get("/api/getrawserverdata")]
fn index_api_getrawserverdata() -> Json<ResponseRawServerData> {
    let cfg = load_config();
    let sd = serde_json::to_string(&cfg.servers).unwrap();
    Json(ResponseRawServerData {
        data: sd,
    })

}
#[rocket::main]
async fn main() {
    let conf = load_config();

    for server in &conf.servers {
        /*if !(is_valid_string(server.name.as_str())) {
            eprint!("Invalid server name: {}", server.name);
            std::process::exit(1);
        }*/
        match is_valid_string(server.name.as_str()) {
            Ok(_) => {}
            Err(e) => {
                eprint!("Invalid server name: {} | Error: {}", server.name, e);
                std::process::exit(1);
            }
        }
    }

    let config = Config {
        address: match conf.addr.parse() {
            Ok(r) => r,
            Err(e) => {
                eprint!("Failed to parse server address: {}", e);
                std::process::exit(1);
            }
        },
        port: conf.port as u16,
        ..Config::default()
    };
    let cors = CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::All)
        .allow_credentials(false)
        .to_cors()
        .unwrap();

    tokio::spawn(async {
        backend::run().await;
    });

    match rocket::custom(config)
        .attach(cors)
        .mount(
            "/",
            routes![
                index_api_servers_servername,
                index_api_serverod_servername,
                index_api_list,
                index_api_getrawserverdata,
            ],
        )
        .launch()
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprint!("Failed to start API Server: {}", e);
        }
    }
}
