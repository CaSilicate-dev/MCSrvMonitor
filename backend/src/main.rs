use rust_mc_status::{McClient, ServerEdition, ServerData};
use sqlite;
use chrono::Utc;
use tokio::time::{sleep,Duration};

fn record(ts: i64,lc: i32,pl: i32){
    let connection = sqlite::open("history.db").unwrap();
    let _ = connection.execute(format!("INSERT INTO mcserver (timestamp, latency, players)
    VALUES ({},{},{})",ts,lc,pl));
}
fn get_time() -> i64{
    let ctimestamp = Utc::now().timestamp();
    return ctimestamp
}
async fn get_data(client: &McClient) -> (i32, i32){
    let status = client.ping("server.fts427.top",ServerEdition::Java).await;
    let latency;
    let players;
    match status{
        Ok(status) => {
            latency = status.latency as i32;
            let data = status.data;
            match data{
                ServerData::Java(status) => {
                    players = status.players.online as i32;
                }
                ServerData::Bedrock(_) => {
                    players = -1;
                }
            }
        }
        Err(_) => {
            latency = -1;
            players = -1;
        }
    }
    return (latency,players)
}
#[tokio::main]
async fn main() {
    let client = McClient::new();
    loop{
        let ct = get_time();
        if ct % 5 == 0 {
            let (l,p) = get_data(&client).await;
            record(ct,l,p);
            sleep(Duration::from_secs(1)).await;
        }
        sleep(Duration::from_millis(500)).await;
    }
    
    
}
