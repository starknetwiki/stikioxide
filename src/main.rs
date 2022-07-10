mod formatters;
mod ipfs_interface;
mod openapi;

use actix_cors::Cors;

// use serde::{Deserialize, Serialize};
use serde_json::Result;

use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};

use crate::openapi::specs::Stiki;

use crate::ipfs_interface::interface::IpfsInterface;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IPFS: IpfsInterface = IpfsInterface::new();
}

async fn add_stiki(_params: web::Json<Stiki>) -> Result<HttpResponse> {
    let resp = r#"
    {
        "status": "ok",
        "message": "string",
        "body": "string"
    }"#;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(resp))
}

async fn modify_stiki(_params: web::Json<Stiki>) -> Result<HttpResponse> {
    let resp = r#"
    {
        "status": "ok",
        "message": "string",
        "body": "string"
    }"#;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(resp))
}

#[get("/v1/list")]
async fn list() -> impl Responder {
    r#"["asd", "fgh"]"#
}

#[get("/v1/list/{peerid}")]
async fn list_peer(_peerid: web::Path<String>) -> impl Responder {
    r#"["asd", "fgh"]"#
}

#[get("/v1/get-stiki/{stikiid}")]
async fn get_stiki(_stiki: web::Path<String>) -> impl Responder {
    let resp = r#"
# This is an example.

bla bla bla"#;
    resp
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PATCH"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/", web::get().to(|| async { "hi!" }))
            .service(web::resource("/v1/add-stiki").route(web::post().to(add_stiki)))
            .service(web::resource("/v1/modify-stiki").route(web::patch().to(modify_stiki)))
            .service(list)
            .service(list_peer)
            .service(get_stiki)
    })
    .bind(("0.0.0.0", 42000))?
    .run()
    .await
}
