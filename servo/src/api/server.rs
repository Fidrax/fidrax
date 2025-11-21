use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{dtos::create_disk_req::CreateDiskRequest, handlers, routes::app::app_routes},
    config::yaml::ServoConfig, service::disk::DiskService,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::disk::create_disk,
    ),
    components(
        schemas(CreateDiskRequest)
    ),
    tags(
        (name = "VMs", description = "VMs management api"),
        (name = "QMP", description = "QMP low level api"),
    )
)]

pub struct ApiDoc;

pub async fn start_http_server(cfg: ServoConfig) -> Result<(), std::io::Error> {
    let host_url = cfg.app.get_host_url();

    let disk_service = DiskService::new(cfg.app.store_path.clone());

    // Start the HTTP server
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(disk_service.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default() // its only for frontend remove it
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
                // .max_age(3600),
            )
            .configure(|cfg| app_routes(cfg))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(host_url)?
    .run()
    .await;

    Ok(())
}
