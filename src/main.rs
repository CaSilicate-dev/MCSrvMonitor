use rocket::{config::Config, serde::json::Json};
use rocket_cors::CorsOptions;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{fs, thread};
mod backend;
mod frontend;
use once_cell::sync::Lazy;
use std::sync::Mutex;
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
    //addr: String,
}
#[derive(Serialize, Debug)]
struct SingleServerData {
    timestamp: i64,
    latency: i32,
    player: i32,
    playerlist: String,
}
impl Default for SingleServerData {
    fn default() -> Self {
        SingleServerData {
            timestamp: 0,
            latency: -1,
            player: -1,
            playerlist: "".to_string(),
        }
    }
}
#[derive(Serialize, Debug, Default)]
struct ResponseData {
    data: Vec<SingleServerData>,
}
#[derive(Serialize, Debug, Default)]
struct ResponseList {
    namelist: Vec<String>,
}
#[macro_use]
extern crate rocket;

static CONFIG: Lazy<Mutex<ConfigFile>> = Lazy::new(|| {
    let conf = load_config();
    Mutex::new(conf)
});

fn load_config() -> ConfigFile {
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
            row.get::<_, i32>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, String>(3)?,
        ))
    }) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Invalid db data {}", e));
        }
    };
    //.unwrap();

    for row in rows {
        let mut single_resp = SingleServerData::default();
        let (ctimestamp, clatency, cplayer, cplayerlist) = row.unwrap();
        single_resp.timestamp = ctimestamp;
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
        resp.namelist.push(server.name);
    }
    Json(resp)
}
#[rocket::main]
async fn main() {
    let conf = load_config();

    for server in &conf.servers {
        if server.name.chars().all(|c| c.is_ascii_lowercase()) {
        } else {
            eprint!("Invalid server name: {}", server.name);
            std::process::exit(1);
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

    thread::spawn(move || {
        frontend::run();
    });

    let _ = rocket::custom(config)
        .attach(cors)
        .mount("/", routes![index_api_servers_servername, index_api_list])
        .launch()
        .await
        .unwrap();
}
