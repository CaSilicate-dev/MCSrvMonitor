use rusqlite::Connection;
use rocket::serde::json::Json;
use serde::Serialize;
#[macro_use] extern crate rocket;

#[derive(Serialize)]
struct RecordData{
    timestamps: Vec<i64>,
    latencys: Vec<i32>,
    players: Vec<i32>,
}

const LENGTH:usize = 100;
fn get_record() -> ([i64; LENGTH], [i32; LENGTH], [i32; LENGTH]){
    let conn = Connection::open("history.db").unwrap();
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

#[get("/data")]
fn index() -> Json<RecordData>{
    let (t,l,p) = get_record();
    let rd = RecordData {
        timestamps: t.to_vec(),
        latencys: l.to_vec(),
        players: p.to_vec(),
    };
    return Json(rd);
}


#[launch]
fn rocket() -> _{
    rocket::build().mount("/", routes![index])
}

/*fn main(){
    let (a,b,c) = get_record();
    println!("{:?},{:?},{:?}",a,b,c);
}*/