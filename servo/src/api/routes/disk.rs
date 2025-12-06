use actix_web::web;

use crate::api::handlers::disk::{create_disk, list_disks, remove_disk, update_disk};

pub fn disk_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/disks")
        .route("/{name}", web::delete().to(remove_disk))
        .route("/{name}/{size}", web::put().to(update_disk))
        .route("", web::post().to(create_disk))
        .route("", web::get().to(list_disks))
    );
}