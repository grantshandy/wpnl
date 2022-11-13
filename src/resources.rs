use std::time::Duration;

use actix_web::{get, web, HttpRequest, HttpResponse};
use log::error;
use prost::Message;
use sysinfo::{CpuExt, SystemExt};
use tokio::time;

use crate::{
    types::{Cpu, Memory, Stats, Swap},
    AppState,
};

#[get("/resources")]
pub async fn broadcast(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> HttpResponse {
    let (resp, mut session, mut _msg_stream) =
        actix_ws::handle(&req, stream).expect("failed to start websocket");

    let system = data.system.clone();
    let tick_length = data.args.tick_length.clone();

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_millis(tick_length)).await;

            system.lock().await.refresh_memory();
            system.lock().await.refresh_cpu();

            let outer_info = system.lock().await;

            let stats = Stats {
                memory: Some(Memory {
                    total_memory: outer_info.total_memory() as i64,
                    available_memory: outer_info.available_memory() as i64,
                    free_memory: outer_info.free_memory() as i64,
                    used_memory: outer_info.used_memory() as i64,
                }),
                swap: Some(Swap {
                    total_swap: outer_info.total_swap() as i64,
                    used_swap: outer_info.used_swap() as i64,
                }),
                cpu: outer_info
                    .cpus()
                    .iter()
                    .map(|cpu| Cpu {
                        usage: cpu.cpu_usage(),
                    })
                    .collect(),
                tick_length: tick_length as i64,
            };

            drop(outer_info);

            let bytes = stats.encode_to_vec();

            if let Err(err) = session.binary(bytes).await {
                error!("error sending to client: {err}");
                break;
            };
        }
    });

    resp
}
