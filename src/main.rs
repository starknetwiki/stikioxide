mod formatters;
mod ipfs_interface;
mod openapi;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use actix_web::{
    get, http::header::ContentType, patch, web, App, HttpResponse, HttpServer, Responder,
};

use crate::openapi::specs::Stiki;
use futures::join;
use ipfs::{make_ipld, Ipfs, IpfsOptions, IpfsPath, TestTypes, UninitializedIpfs};

#[macro_use]
use lazy_static::lazy_static;
use tokio::task;
use crate::ipfs_interface::interface::IpfsInterface;

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
async fn list_peer(peerid: web::Path<String>) -> impl Responder {
    r#"["asd", "fgh"]"#
}

#[get("/v1/get-stiki/{stikiid}")]
async fn get_stiki(stiki: web::Path<String>) -> impl Responder {
    let resp = r#"
# This is an example.

bla bla bla"#;
    resp
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "hi!" }))
            .service(web::resource("/v1/add-stiki").route(web::post().to(add_stiki)))
            .service(web::resource("/v1/modify-stiki").route(web::patch().to(modify_stiki)))
            .service(web::resource("/v1/list").route(web::post().to(modify_stiki)))
            .service(list)
            .service(list_peer)
            .service(get_stiki)
    })
    .bind(("0.0.0.0", 42000))?
    .run()
    .await
}