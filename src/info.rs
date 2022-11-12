use actix_web::{get, HttpResponse};
use log::error;
use serde::Serialize;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
struct Info {
    name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
}

#[get("/info")]
pub async fn info() -> HttpResponse {
    let system = System::default();

    let info = Info {
        name: system.name(),
        kernel_version: system.kernel_version(),
        os_version: system.os_version(),
        host_name: system.host_name(),
    };

    let json: String = match serde_json::to_string(&info) {
        Ok(json) => json,
        Err(err) => {
            error!("Error parsing info json: {err}");

            "{}".to_string()
        }
    };

    HttpResponse::Ok().body(json)
}
