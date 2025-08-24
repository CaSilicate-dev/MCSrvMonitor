use crate::config::Config;
use rocket::fs::{FileServer, Options, relative};
use rocket::{Build, Rocket, get, routes};
use rocket_dyn_templates::{Template, context};
use rusqlite::Connection;
use serde_yaml;
use std::fs;
use std::sync::Arc;

const LENGTH: usize = 100;

fn get_record(filename: &str) -> ([i64; LENGTH], [i32; LENGTH], [i32; LENGTH]) {
    let conn = Connection::open(filename).unwrap();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT * FROM mcserver ORDER BY timestamp DESC LIMIT {}",
            LENGTH
        ))
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0).unwrap(),
                row.get::<_, i32>(1).unwrap(),
                row.get::<_, i32>(2).unwrap(),
            ))
        })
        .unwrap();
    let mut timestamps: [i64; LENGTH] = [0; LENGTH];
    let mut latencys: [i32; LENGTH] = [0; LENGTH];
    let mut players: [i32; LENGTH] = [0; LENGTH];
    let mut i = 0;
    for row in rows {
        let (ctimestamp, clatency, cplayer) = row.unwrap();
        timestamps[i] = ctimestamp;
        latencys[i] = clatency;
        players[i] = cplayer;
        i += 1;
    }
    (timestamps, latencys, players)
}

fn advanced_round(value: f64, digits: u32) -> f64 {
    let m = value * 10_f64.powi(digits as i32);
    let r = m.round() / 10_f64.powi(digits as i32);
    r
}

fn load_lang(path: &str) -> serde_yaml::Value {
    let data = fs::read_to_string(path).unwrap();
    let v = serde_yaml::from_str(&data).unwrap();
    v
}

fn generate_data(filename: &str) -> (String, String, String, String, String) {
    let lang = load_lang("assets/lang.yaml");
    let (_, latencys, _) = get_record(filename);

    let current_latency = latencys[0];
    let (current_status, current_status_color) = if current_latency >= 0 && current_latency <= 150 {
        (lang["online"].as_str().unwrap().to_string(), "#90ee90")
    } else if current_latency > 150 {
        (lang["hl"].as_str().unwrap().to_string(), "#ffff00")
    } else {
        (lang["offline"].as_str().unwrap().to_string(), "#ff0000")
    };

    let mut sum = 0;
    for i in latencys.iter() {
        sum += *i;
    }
    let rate = advanced_round((sum as f64) / (LENGTH as f64), 3);
    let rate_color = if rate >= 90_f64 {
        "#90ee90"
    } else if rate < 90_f64 && rate >= 50_f64 {
        "#ffff00"
    } else {
        "#ff0000"
    };

    let mut verbose_info: String = "".to_string();
    for i in latencys.iter() {
        let color = if *i >= 0 && *i <= 150 {
            "#90ee90"
        } else if *i > 150 {
            "#ffff00"
        } else {
            "#ff0000"
        };
        verbose_info.push_str(
            format!(
                r#"<span class="block" style="color : {};">{}</span>"#,
                color,
                lang["block"].as_str().unwrap()
            )
            .as_str(),
        );
    }
    (
        current_status_color.to_string(),
        current_status.to_string(),
        rate_color.to_string(),
        format!("{}", rate),
        verbose_info,
    )
}

#[get("/data")]
fn root_data(config: &rocket::State<Arc<Config>>) -> Template {
    let (color1, status, color2, rate, verbose) = generate_data(&config.db);
    Template::render(
        "index",
        context! {color1: color1, status: status, color2: color2, rate: rate, verbose: verbose},
    )
}

pub fn rocket(config: Arc<Config>) -> Rocket<Build> {
    rocket::custom(rocket::Config {
        address: config.web.addr.parse().unwrap(),
        port: config.web.port,
        ..Default::default()
    })
    .manage(config)
    .attach(Template::fairing())
    .mount(
        "/assets",
        FileServer::new(relative!("/assets"), Options::Missing | Options::NormalizeDirs),
    )
    .mount("/", routes![root_data])
}
