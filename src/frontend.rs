use rocket::config::Config;
use rocket::fs::{FileServer, relative};
use rocket_cors::CorsOptions;

pub fn run(addr: String, port: u16) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let config = Config {
                address: match addr.parse() {
                    Ok(r) => r,
                    Err(e) => {
                        eprint!("Failed to parse server address: {}", e);
                        std::process::exit(1);
                    }
                },
                port: port,
                ..Config::default()
            };
            let cors = CorsOptions::default()
                .allowed_origins(rocket_cors::AllowedOrigins::All)
                .allow_credentials(false)
                .to_cors()
                .unwrap();

            let _ = match rocket::custom(config)
                .attach(cors)
                .mount("/", FileServer::from(relative!("./frontend/build")))
                .launch()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    eprint!("Failed to start webui:{}", e);
                    std::process::exit(1);
                }
            };
        });
}
