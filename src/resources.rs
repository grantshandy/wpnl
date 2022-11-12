use std::{sync::Arc, time::Duration};

use actix_web::{get, web, Responder};
use actix_web_lab::sse::{self, Data};
use log::error;
use serde::Serialize;
use sysinfo::{System, SystemExt};
use tokio::{sync::Mutex, time};

use crate::AppState;

#[derive(Serialize)]
struct Stats {
    total_memory: u64,
    free_memory: u64,
    available_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}

#[get("/resources")]
pub async fn broadcast(data: web::Data<AppState>) -> impl Responder {
    let (tx, rx) = sse::channel(2);

    let info = Arc::new(Mutex::new(System::default()));

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_millis(data.args.tick_length)).await;

            info.lock().await.refresh_memory();

            let outer_info = info.lock().await;

            let system = Stats {
                total_memory: outer_info.total_memory(),
                free_memory: outer_info.free_memory(),
                available_memory: outer_info.available_memory(),
                used_memory: outer_info.used_memory(),
                total_swap: outer_info.total_swap(),
                used_swap: outer_info.used_swap(),
            };

            drop(outer_info);

            let json = match serde_json::to_string(&system) {
                Ok(data) => data,
                Err(err) => {
                    error!("Error serializing sysinfo as json: {err}");

                    "{}".to_string()
                }
            };

            let data = Data::new(json);

            if let Err(err) = tx.send(data).await {
                if err.to_string() != "channel closed" {
                    error!("Error sending sysinfo to user: {err}");
                }

                break;
            };
        }
    });

    return rx;
}
