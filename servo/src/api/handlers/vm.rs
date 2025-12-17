use actix_web::{HttpResponse, Responder, web};

use crate::{api::dtos::{create_vm_req::CreateVMRequest, vm_config::ResponseQemuConfig}, service::{disk::DiskService, vm::QemuVMService}};

#[utoipa::path(
    post,
    path = "/api/v1/vm",
    request_body = CreateVMRequest,
    responses(
        (status = 200, description = "vm created successfully"),
        (status = 400, description = "failed to create vm")
    ),
    tag = "VMs"
)]
pub async fn create_vm(
    req: web::Json<CreateVMRequest>,
    svc: web::Data<QemuVMService>,
) -> impl Responder {
    match svc.create_vm(req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("vm created successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/vm/start/{name}",
    params(
        ("name" = String, Path, description = "name of the vm")
    ),
    responses(
        (status = 200, description = "vm started successfully"),
        (status = 400, description = "failed to start vm")
    ),
    tag = "VMs"
)]
pub async fn start_vm(name: web::Path<String>, svc: web::Data<QemuVMService>) -> impl Responder {
    match svc.start(&name.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("vm started successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/vm/shutdown/{name}",
    params(
        ("name" = String, Path, description = "name of the vm")
    ),
    responses(
        (status = 200, description = "vm shutdown successfully"),
        (status = 400, description = "failed to start vm")
    ),
    tag = "VMs"
)]
pub async fn shutdown_vm(name: web::Path<String>, svc: web::Data<QemuVMService>) -> impl Responder {
    match svc.shutdown(&name.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("vm shutdown successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/vm/restart/{name}",
    params(
        ("name" = String, Path, description = "name of the vm")
    ),
    responses(
        (status = 200, description = "vm shutdown successfully"),
        (status = 400, description = "failed to restart vm")
    ),
    tag = "VMs"
)]
pub async fn restart_vm(name: web::Path<String>, svc: web::Data<QemuVMService>) -> impl Responder {
    match svc.restart(&name.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("vm restart successfully"),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/vm/status/{name}",
    params(
        ("name" = String, Path, description = "name of the vm")
    ),
    responses(
        (status = 200, description = "vm shutdown successfully"),
        (status = 400, description = "failed to restart vm")
    ),
    tag = "VMs"
)]
pub async fn status_vm(name: web::Path<String>, svc: web::Data<QemuVMService>) -> impl Responder {
    match svc.status(&name.into_inner()).await {
        Ok(vm_state) => {
            let state = match vm_state {
                enginseer::vm::backend::qemu::state::VMState::Stopped => "stopped",
                enginseer::vm::backend::qemu::state::VMState::Running => "running",
                enginseer::vm::backend::qemu::state::VMState::Paused => "paused",
                enginseer::vm::backend::qemu::state::VMState::Error => "error",
                enginseer::vm::backend::qemu::state::VMState::Unknown => "unknown",
            };

            HttpResponse::Ok().json(state)
        }
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/vm",
    responses(
        (status = 200, description = "vm list successfully", body=[Vec<ResponseQemuConfig>]),
        (status = 400, description = "failed to get vm list")
    ),
    tag = "VMs"
)]
pub async fn list_vms(svc: web::Data<QemuVMService>) -> impl Responder {
    match svc.list().await {
        Ok(qemu_configs) => {
            let vms: Vec<ResponseQemuConfig> = qemu_configs
                .into_iter()
                .map(ResponseQemuConfig::from)
                .collect();
            HttpResponse::Ok().json(vms)
        }
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}
