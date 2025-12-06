use actix_web::web;

use crate::api::routes::{disk::disk_routes, vm::vm_routes};

pub fn app_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(disk_routes)
            .configure(vm_routes),
    );
}
