use actix_web::{get, web::Data, HttpResponse};
use prost::Message;
use sysinfo::SystemExt;

use crate::{types::Info, AppState};

#[get("/info")]
pub async fn info(data: Data<AppState>) -> HttpResponse {
    let system = data.system.lock().await;

    let info = Info {
        name: system.name(),
        kernel_version: system.kernel_version(),
        os_version: system.os_version(),
        host_name: system.host_name(),
    };

    drop(system);

    HttpResponse::Ok().body(info.encode_to_vec())
}
