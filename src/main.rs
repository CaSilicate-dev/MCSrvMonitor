use std::sync::Arc;
use mcsrv_monitor::config::Config;
use mcsrv_monitor::collector::run;
use mcsrv_monitor::web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::load("config.yaml")?;
    let conf_arc = Arc::new(conf);
    run(&conf_arc).await;
    web::rocket(conf_arc).launch().await?;
    Ok(())
}
