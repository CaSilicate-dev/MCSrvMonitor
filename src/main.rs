use chrono::{DateTime, Local, TimeZone};
use once_cell::sync::Lazy;
use regex::Regex;
use rocket::{config::Config, serde::json::Json};
use rocket_cors::CorsOptions;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
mod backend;

#[derive(Deserialize, Clone)]
struct ConfigFile {
    addr: String,
    port: u16,
    length: u32,
    backend: BackendConfig,
    //frontend: FrontendConfig,
    servers: Vec<SingleServerConfig>,
}
/*#[derive(Debug, Deserialize, Clone)]
struct FrontendConfig {
    addr: String,
    port: u16,
}*/
#[derive(Debug, Deserialize, Clone)]
struct BackendConfig {
    dbfile: String,
    //interval: u32,
}
#[derive(Debug, Deserialize, Clone)]
struct SingleServerConfig {
    name: String,
    label: String,
    //addr: String,
    #[serde(rename = "type")]
    stype: i8,
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

#[macro_use]
extern crate rocket;

static CONFIG: Lazy<Mutex<ConfigFile>> = Lazy::new(|| {
    let conf = load_config();
    Mutex::new(conf)
});

fn is_valid_string(s: &str) -> Result<bool, regex::Error> {
    let aaa: Regex = Regex::new(r"^[A-Za-z0-9_]+$")?;
    return Ok(aaa.is_match(s));
}

fn load_config() -> ConfigFile {
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
    let conf = CONFIG.lock().unwrap().clone();
    match get_record(servername, conf.backend.dbfile, conf.servers, conf.length) {
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
    let conf = CONFIG.lock().unwrap().clone();
    match get_record(servername, conf.backend.dbfile, conf.servers, 1) {
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
                index_api_list
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
