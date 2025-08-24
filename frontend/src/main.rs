use rusqlite::Connection;
use serde_json;
use serde_yaml;
use std::fs;
use serde::Deserialize;
use rocket_dyn_templates::{Template, context};
use rocket::config::{Config};
const LENGTH:usize = 100;

#[derive(Deserialize)]
struct ConfigData{
    database_filename: String,
    port: u32,
    addr: String,
}

#[macro_use] extern crate rocket;

fn load_config() -> ConfigData{
    let contents = fs::read_to_string("config.yaml").unwrap();
    let config: ConfigData = serde_yaml::from_str(&contents).unwrap();
    return config;
}

fn get_record(filename: String) -> ([i64; LENGTH], [i32; LENGTH], [i32; LENGTH]){
    let conn = Connection::open(filename).unwrap();
    let mut stmt = conn.prepare(&format!("SELECT * FROM mcserver ORDER BY timestamp DESC LIMIT {}",LENGTH)).unwrap();
    
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0).unwrap(),
            row.get::<_, i32>(1).unwrap(),
            row.get::<_, i32>(2).unwrap(),
        ))
    }).unwrap();
    let mut timestamps: [i64; LENGTH] = [0; LENGTH];
    let mut latencys: [i32; LENGTH] = [0; LENGTH];
    let mut players: [i32; LENGTH] = [0; LENGTH];
    let mut i = 0;
    for row in rows {
        let (ctimestamp,clatency,cplayer) = row.unwrap();
        timestamps[i] = ctimestamp;
        latencys[i] = clatency;
        players[i] = cplayer;
        //println!("{id} {name} {timestamp}");
        i +=1;
    }
    return (timestamps,latencys,players);
}

fn advanced_round(value: f64, digits: u32) -> f64{
    let m = value * 10_f64.powi(digits as i32);
    let r = m.round() / 10_f64.powi(digits as i32);
    return r
}

fn load_lang(path: &str) -> serde_json::Value{
    let data = fs::read_to_string(path).unwrap();
    let v = serde_json::from_str(&data).unwrap();
    return v;
}
fn generate_data(filename: String) -> (String, String, String, String, String, ){
    let lang = load_lang("assets/lang.json");

    let (_, latencys, _) = get_record(filename);

    let current_latency = latencys[0];
    let current_status;
    let current_status_color;
    if current_latency >=0 && current_latency <= 150 {
        current_status = (&lang["online"].as_str().unwrap()).to_string();
        current_status_color = "#90ee90";
    }
    else if current_latency > 150 {
        current_status = (&lang["hl"].as_str().unwrap()).to_string();
        current_status_color = "#ffff00";
    }
    else {
        current_status = (&lang["offline"].as_str().unwrap()).to_string();
        current_status_color = "#ff0000";
    }
    let rate;
    let mut sum = 0;
    let rate_color;
    for i in latencys.iter(){
        sum += *i;
    }
    rate = advanced_round((sum as f64) / (LENGTH as f64), 3);
    if rate >= 90_f64 {
        rate_color = "#90ee90";
    }
    else if rate < 90_f64 && rate >= 50_f64 {
        rate_color = "#ffff00";
    }
    else {
        rate_color = "#ff0000";
    }

    let mut verbose_info: String = "".to_string();

    for i in latencys.iter() {
        if *i >= 0 && *i <= 150 {
            verbose_info.push_str(format!("<span class=\"block\" style=\"color : {};\">{}</span>", "#90ee90",lang["block"].as_str().unwrap()).as_str());
        }
        else if *i > 150 {
            verbose_info.push_str(format!(r#"<span class="block" style="color : {};">{}</span>"#, "#ffff00",lang["block"].as_str().unwrap()).as_str());
        }
        else {
            verbose_info.push_str(format!(r#"<span class="block" style="color : {};">{}</span>"#, "#ff0000",lang["block"].as_str().unwrap()).as_str());
        }
    }
    return (current_status_color.to_string(), current_status.to_string(), rate_color.to_string(), format!("{}",rate), verbose_info);

}

#[get("/data")]
fn root_data() -> Template{
    let conf = load_config();

    let (color1, status, color2, rate, verbose) = generate_data(conf.database_filename);
    Template::render("index", context! {color1: color1, status: status, color2: color2, rate: rate, verbose: verbose})
}

#[rocket::main]
async fn main(){
    let conf = load_config();
    let config = Config {
        address: conf.addr.parse().unwrap(),
        port: conf.port as u16,
        ..Config::default()
    };
    let _ = rocket::custom(config)
        .attach(Template::fairing())
        .mount("/", routes![root_data])
        .launch().await.unwrap();
}

