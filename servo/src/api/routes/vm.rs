use actix_web::web;

use crate::api::handlers::vm::{create_vm, restart_vm, shutdown_vm, start_vm, status_vm};

pub fn disk_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/vm")
            .route("/start/{name}", web::put().to(start_vm))
            .route("/shutdown/{name}", web::put().to(shutdown_vm))
            .route("/restart/{name}", web::put().to(restart_vm))
            .route("/status/{name}", web::put().to(status_vm))
            .route("", web::post().to(create_vm)), // .route("/", web::get().to(list_vms))
    );
}
