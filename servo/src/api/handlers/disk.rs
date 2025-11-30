use actix_web::{HttpResponse, Responder, web};

use crate::{
    api::dtos::{create_disk_req::CreateDiskRequest, disk_config::ResponseQcow2DiskConfig},
    service::disk::DiskService,
};

#[utoipa::path(
    post,
    path = "/api/v1/disks",
    request_body = CreateDiskRequest,
    responses(
        (status = 200, description = "Disk created successfully"),
        (status = 400, description = "Failed to create disk")
    ),
    tag = "Disks"
)]
pub async fn create_disk(
    req: web::Json<CreateDiskRequest>,
    svc: web::Data<DiskService>,
) -> impl Responder {
    match svc.create_disk(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("disk created successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/disks/{name}",
    params(
        ("name" = String, Path, description = "name of the disk")
    ),
    responses(
        (status = 200, description = "Disk removed successfully"),
        (status = 404, description = "Disk not found"),
        (status = 400, description = "Failed to remove disk")
    ),
    tag = "Disks"
)]
pub async fn remove_disk(name: web::Path<String>, svc: web::Data<DiskService>) -> impl Responder {
    match svc.remove_disk(name.into_inner().as_str()).await {
        Ok(_) => HttpResponse::Ok().body("disk removed successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/disks/{name}/{size}",
    params(
        ("name" = String, Path, description = "name of the disk"),
        ("size" = u64, Path, description = "size of the disk")
    ),
    responses(
        (status = 200, description = "Disk updated successfully"),
        (status = 400, description = "Failed to update disk")
    ),
    tag = "Disks"
)]
pub async fn update_disk(
    name_size: web::Path<(String, u64)>,
    svc: web::Data<DiskService>,
) -> impl Responder {
    let (name, size) = name_size.into_inner();
    match svc.update_disk(name.as_str(), size).await {
        Ok(_) => HttpResponse::Ok().body("disk updated successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/disks",
    responses(
        (status = 200, description = "Disk removed successfully", body=[Vec<ResponseQcow2DiskConfig>]),
        (status = 400, description = "Failed to remove disk")
    ),
    tag = "Disks"
)]
pub async fn list_disks(svc: web::Data<DiskService>) -> impl Responder {
    match svc.list_disks().await {
        Ok(qcow_configs) => {
            let disks: Vec<ResponseQcow2DiskConfig> = qcow_configs
                .into_iter()
                .map(ResponseQcow2DiskConfig::from)
                .collect();
            HttpResponse::Ok().json(disks)
        }
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}
