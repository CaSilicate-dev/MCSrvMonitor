use crate::config::{self, Config, ServerConfig};
use chrono::Utc;
use rust_mc_status::{McClient, ServerEdition};
use rusqlite::{Connection, params};
use std::sync::Arc;
use tokio::time::{Duration, sleep};

pub async fn run(conf: &Arc<Config>) {
    let db = Arc::new(conf.db.clone());
    let interval = conf.interval;
    let server = ServerConfig {
        java: conf.mc.java,
        addr: Arc::new(conf.mc.addr.clone()).to_string(),
    };
    tokio::spawn(async move {
        let conn = match Connection::open(&*db) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Database connection failed: {}", e);
                return;
            }
        };
        loop {
            match ping_server(&server, interval).await {
                Ok((latency, players)) => {
                    let ts = Utc::now().timestamp();
                    if let Err(e) = conn.execute(
                        "INSERT INTO mcserver (timestamp, latency, players) VALUES (?1, ?2, ?3)",
                        params![ts, latency, players],
                    ) {
                        eprintln!("Write data failed: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Minecraft server ping failed: {}", e);
                }
            }
            sleep(Duration::from_secs(interval)).await;
        }
    });
}

async fn ping_server(
    conf: &config::ServerConfig,
    timeout: u64,
) -> Result<(u64, i64), Box<dyn std::error::Error>> {
    let addr = conf.addr.as_str();
    let timeout = std::time::Duration::from_secs(timeout);

    let client = McClient::new().with_timeout(timeout).with_max_parallel(16);
    if conf.java {
        let status = client.ping(addr, ServerEdition::Java).await?;
        Ok((status.latency as u64, status.online as i64))
    } else {
        let status = client.ping(addr, ServerEdition::Bedrock).await?;
        Ok((status.latency as u64, status.online as i64))
    }
}
