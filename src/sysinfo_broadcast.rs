use actix_web::{get, Responder};
use actix_web_lab::sse;
use log::error;
use serde::Serialize;
use tokio::time::{self, Duration};

#[derive(Serialize)]
struct SystemInfo {
    used_memory: u64,
}

#[get("/sysinfo-broadcast")]
pub async fn broadcast() -> impl Responder {
    let (tx, rx) = sse::channel(2);

    tokio::spawn(async move {
        time::sleep(Duration::from_millis(100)).await;

        let info = SystemInfo {
            used_memory: 1235345,
        };

        let info_json = serde_json::to_string(&info).unwrap();

        if let Err(err) = tx.send(sse::Data::new(info_json)).await {
            error!("Error broadcasting sysinfo to client: {err}");
        }
    });

    rx
}
