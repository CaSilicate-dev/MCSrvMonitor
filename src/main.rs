use rocket::{config::Config, serde::json::Json};
use rocket_cors::CorsOptions;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json;
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
        SingleServerData { timestamp: 0, latency: -1, player: -1, playerlist: "".to_string() }
    }
}
#[derive(Serialize, Debug)]
struct ResponseData {
    data: Vec<SingleServerData>,
}
impl Default for ResponseData {
    fn default() -> Self {
        ResponseData { data: Vec::new() }
    }
}
#[derive(Serialize, Debug)]
struct ResponseList {
    namelist: Vec<String>,
}
impl Default for ResponseList {
    fn default() -> Self {
        ResponseList { namelist: Vec::new() }
    }
}
#[macro_use]
extern crate rocket;

static CONFIG: Lazy<Mutex<ConfigFile>> = Lazy::new(|| {
    let conf = load_config();
    return Mutex::new(conf)
});

fn load_config() -> ConfigFile {
    //let contents = fs::read_to_string("config.yaml").unwrap();
    let configfile = fs::read_to_string("config.json");
    let contents = match configfile {
        Ok(r) => {
            r
        }
        Err(e) => {
            eprint!("Failed to open essential config: {} \n", e);
            
            std::process::exit(1);
        }
    };
    let cont = serde_json::from_str(&contents);
    let config: ConfigFile;
    match cont {
        Ok(r) => {
            config = r;
        }
        Err(e) => {
            eprint!("Failed to parse essential config: {}\n", e);
            std::process::exit(1);
        }
    }

    return config;
}

/*fn advanced_round(value: f64, digits: u32) -> f64 {
    let m = value * 10_f64.powi(digits as i32);
    let r = m.round() / 10_f64.powi(digits as i32);
    return r;
}*/

/*fn get_record(filename: String, length: u32) -> (Vec<i64>, Vec<i32>, Vec<i32>) {
    let conn = match Connection::open(filename) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Failed to open database: {}\n", e);
            std::process::exit(1);
        }
    };
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mcserver (
            timestamp INTEGER PRIMARY KEY,
            latency INTEGER NOT NULL,
            players INTEGER NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");

    let mut stmt = match conn.prepare(&format!(
        "SELECT * FROM mcserver ORDER BY timestamp DESC LIMIT {}",
        length
    )) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Failed to read database: {}\n", e);
            std::process::exit(1);
        }
    };
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0).unwrap(),
                row.get::<_, i32>(1).unwrap(),
                row.get::<_, i32>(2).unwrap(),
            ))
        })
        .unwrap();
    let mut timestamps: Vec<i64> = Vec::new();
    let mut latencys: Vec<i32> = Vec::new();
    let mut players: Vec<i32> = Vec::new();
    for row in rows {
        let (ctimestamp, clatency, cplayer) = row.unwrap();
        timestamps.push(ctimestamp);
        latencys.push(clatency);
        players.push(cplayer);
        //println!("{id} {name} {timestamp}");
    }
    return (timestamps, latencys, players);
}



fn load_lang(path: &str) -> serde_json::Value {
    let data = match fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Failed to read language file: {}", e);
            r#"{"online": "Online", "offline": "Offline", "hl": "High Latency", "block": "■"}"#
                .to_string()
        }
    };
    let v = match serde_json::from_str(&data) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Failed to read language file: {}\n", e);
            std::process::exit(1);
        }
    };
    return v;
}
fn generate_data(filename: String, length: u32) -> MonitorData {
    let lang = load_lang("assets/lang.json");

    let (ts, latencys, _) = get_record(filename, length);

    let current_latency = latencys[0];
    let current_status;
    let current_status_color;
    if current_latency >= 0 && current_latency <= 150 {
        current_status = (&lang["online"].as_str().unwrap()).to_string();
        current_status_color = "#90ee90";
    } else if current_latency > 150 {
        current_status = (&lang["hl"].as_str().unwrap()).to_string();
        current_status_color = "#ffff00";
    } else {
        current_status = (&lang["offline"].as_str().unwrap()).to_string();
        current_status_color = "#ff0000";
    }
    let rate;
    let mut sum = 0;
    let rate_color;
    for i in latencys.iter() {
        if *i >= 0 {
            sum += 100;
        }
    }
    rate = advanced_round((sum as f64) / (latencys.len() as f64), 3);
    if rate >= 90_f64 {
        rate_color = "#90ee90";
    } else if rate < 90_f64 && rate >= 50_f64 {
        rate_color = "#ffff00";
    } else {
        rate_color = "#ff0000";
    }

    let mut verbose_info: String = "".to_string();

    for i in 0..latencys.len() {
        if latencys[i] >= 0 && latencys[i] <= 150 {
            verbose_info.push_str(
                format!(
                    "<span title=\"{}\" class=\"block\" style=\"color : {};\">{}</span>",
                    ts[i].to_string(),
                    "#90ee90",
                    lang["block"].as_str().unwrap()
                )
                .as_str(),
            );
        } else if latencys[i] > 150 {
            verbose_info.push_str(
                format!(
                    "<span title=\"{}\" class=\"block\" style=\"color : {};\">{}</span>",
                    ts[i].to_string(),
                    "#ffff00",
                    lang["block"].as_str().unwrap()
                )
                .as_str(),
            );
        } else {
            verbose_info.push_str(
                format!(
                    "<span title=\"{}\" class=\"block\" style=\"color : {};\">{}</span>",
                    ts[i].to_string(),
                    "#ff0000",
                    lang["block"].as_str().unwrap()
                )
                .as_str(),
            );
        }
    }
    /*return (
        current_status_color.to_string(),
        current_status.to_string(),
        rate_color.to_string(),
        format!("{}", rate),
        verbose_info,
    );*/
    return MonitorData {
        color1: current_status_color.to_string(),
        current: current_status.to_string(),
        color2: rate_color.to_string(),
        rate: format!("{}", rate).to_string(),
        verboseinfo: verbose_info,
    };
}

#[get("/api/getdata")]
fn root_data() -> Json<ApiResponse> {
    let conf = load_config();

    let md = generate_data(conf.backend.dbfile, conf.length);

    return Json(ApiResponse {
        code: 200,
        message: "ok".to_string(),
        data: md,
    });
}*/

fn get_record(servername:String, filename: String, servers: Vec<SingleServerConfig>, length: u32) -> Result<ResponseData, String> {
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
        server.name,
        length,
    )) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Failed to read database {}", e));
        }
    };
    let rows = match stmt
    .query_map([], |row| {
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
    return Ok(resp);
}

#[get("/api/servers/<servername>")]
fn index_api_servers_servername(servername:String) -> Json<ResponseData> {
    let conf = CONFIG.lock().unwrap().clone();
    match get_record(servername, conf.backend.dbfile, conf.servers, conf.length) {
        Ok(r) => {
            return Json(r)
        }
        Err(e) => {
            return Json(ResponseData { data: vec![SingleServerData{playerlist: e,..Default::default()}]})
        }
    }
}
#[get("/api/list")]
fn index_api_list() -> Json<ResponseList> {
    let conf = load_config();
    let mut resp = ResponseList::default();
    for server in conf.servers {
        resp.namelist.push(server.name);
    }
    return Json(resp);
}
#[rocket::main]
async fn main() {
    let conf = load_config();

    for server in &conf.servers {
        if server.name.chars().all(|c| c.is_ascii_lowercase()) {}
        else {
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