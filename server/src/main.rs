use rocket::{config::Config, serde::json::Json};
use rocket_cors::{CorsOptions};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::{fs, thread};
mod backend;

#[derive(Deserialize)]
struct ConfigData {
    database_filename: String,
    port: u32,
    addr: String,
    length: u32,
}

#[derive(Serialize)]
struct ApiResponse {
    code: u32,
    message: String,
    data: MonitorData,
}
#[derive(Serialize)]
struct MonitorData {
    color1: String,
    current: String,
    color2: String,
    rate: String,
    verboseinfo: String,
}
#[macro_use]
extern crate rocket;

fn load_config() -> ConfigData {
    let contents = match fs::read_to_string("config.yaml") {
        Ok(r) => {
            r
        }
        Err(e) => {
            eprint!("Error: {}\n",e);
            panic!("Failed to open essential config");
        }
    };
    let config: ConfigData = match serde_yaml::from_str(&contents) {
        Ok(r) => {
            r
        }
        Err(e) => {
            eprint!("Error: {}\n",e);
            panic!("Failed to open essential config");
        }
    };
    return config;
}

fn get_record(filename: String, length: u32) -> (Vec<i64>, Vec<i32>, Vec<i32>) {
    let conn = match Connection::open(filename) {
        Ok(r) => {
            r
        }
        Err(e) => {
            eprint!("Error: {}\n",e);
            panic!("Failed to open database");
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

    let mut stmt = match conn
        .prepare(&format!(
            "SELECT * FROM mcserver ORDER BY timestamp DESC LIMIT {}",
            length
        )) {
            Ok(r) => {
                r
            }
            Err(e) => {
                eprint!("Error: {}\n",e);
                panic!("Failed to read database");
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

fn advanced_round(value: f64, digits: u32) -> f64 {
    let m = value * 10_f64.powi(digits as i32);
    let r = m.round() / 10_f64.powi(digits as i32);
    return r;
}

fn load_lang(path: &str) -> serde_json::Value {
    let data = match fs::read_to_string(path) {
        Ok(r) => {
            r
        }
        Err(e) => {
            eprint!("Failed to read language file: {}",e);
            r#"{"online": "Online", "offline": "Offline", "hl": "High Latency", "block": "■"}"#.to_string()
        }
    };
    let v = match serde_json::from_str(&data) {
        Ok(r) => {
            r
        }
        Err(e) => {
            eprint!("Error: {}\n",e);
            panic!("Failed to read language filee");
        }
    };
    return v;
}
fn generate_data(filename: String, length: u32) -> MonitorData {
    let lang = load_lang("assets/lang.json");

    let (_, latencys, _) = get_record(filename, length);

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
    rate = advanced_round((sum as f64) / (length as f64), 3);
    if rate >= 90_f64 {
        rate_color = "#90ee90";
    } else if rate < 90_f64 && rate >= 50_f64 {
        rate_color = "#ffff00";
    } else {
        rate_color = "#ff0000";
    }

    let mut verbose_info: String = "".to_string();

    for i in latencys.iter() {
        if *i >= 0 && *i <= 150 {
            verbose_info.push_str(
                format!(
                    "<span class=\"block\" style=\"color : {};\">{}</span>",
                    "#90ee90",
                    lang["block"].as_str().unwrap()
                )
                .as_str(),
            );
        } else if *i > 150 {
            verbose_info.push_str(
                format!(
                    r#"<span class="block" style="color : {};">{}</span>"#,
                    "#ffff00",
                    lang["block"].as_str().unwrap()
                )
                .as_str(),
            );
        } else {
            verbose_info.push_str(
                format!(
                    r#"<span class="block" style="color : {};">{}</span>"#,
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
            verboseinfo: verbose_info
        }
}

#[get("/data")]
fn root_data() -> Json<ApiResponse> {
    let conf = load_config();

    let md = generate_data(conf.database_filename, conf.length);
    
    return Json(ApiResponse {
        code: 200,
        message: "ok".to_string(),
        data: md
    });
}

#[rocket::main]
async fn main() {
    let conf = load_config();
    let config = Config {
        address: match conf.addr.parse() {
            Ok(r) => {
                r
            }
            Err(e) => {
                eprint!("Error: {}",e);
                panic!("Failed to parse server address")
            }
        },
        port: conf.port as u16,
        ..Config::default()
    };
    let cors = CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::All)
        .allow_credentials(false)
        .to_cors().unwrap();
    thread::spawn(|| {
        backend::run();
    });

    let _ = rocket::custom(config)
        .attach(cors)
        .mount("/", routes![root_data])
        .launch()
        .await
        .unwrap();
}
