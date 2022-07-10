mod formatters;
mod ipfs_interface;
mod openapi;

use once_cell::sync::OnceCell;
use std::collections::HashMap;

use actix_cors::Cors;

use serde_json::Result;

use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder};

use crate::ipfs_interface::file_structure::StikiPage;
use crate::ipfs_interface::interface::IpfsInterface;
use crate::openapi::specs::{NewStikiResp, StatusEnum, Stiki};

static IPFS: OnceCell<IpfsInterface> = OnceCell::new();

async fn add_stiki(params: web::Json<Stiki>) -> Result<HttpResponse> {
    match IPFS.get() {
        Some(ipfs) => {
            let mut ref_files: HashMap<String, String> = HashMap::new();
            for (file_ref, ref_value) in params.refs.iter() {
                ref_files.insert(
                    file_ref.clone().to_owned().to_string(),
                    ipfs.add_file(ref_value.to_owned()).await,
                );
            }
            ipfs.add_file(params.body.clone()).await;
            let stikipage = StikiPage {
                body: params.body.clone(),
                refs: params.refs.clone(),
            };
            let stiki_wiki = ipfs
                .add_file(serde_json::to_string(&stikipage).unwrap())
                .await;
            Ok(HttpResponse::Ok().content_type("application/json").body(
                serde_json::to_string(&NewStikiResp {
                    status: StatusEnum::OK.as_str().to_owned(),
                    message: Some("Success".to_owned()),
                    body: Some(stiki_wiki),
                })
                .unwrap(),
            ))
        }
        _ => Ok(HttpResponse::Gone().content_type("application/json").body(
            serde_json::to_string(&NewStikiResp {
                status: StatusEnum::ERR.as_str().to_owned(),
                message: Some("Page could not be added.".to_owned()),
                body: None,
            })
            .unwrap(),
        )),
    }
}

async fn modify_stiki(params: web::Json<Stiki>) -> Result<HttpResponse> {
    match IPFS.get() {
        Some(ipfs) => {
            let mut ref_files: HashMap<String, String> = HashMap::new();
            for (file_ref, ref_value) in params.refs.iter() {
                ref_files.insert(
                    file_ref.clone().to_owned().to_string(),
                    ipfs.add_file(ref_value.to_owned()).await,
                );
            }
            ipfs.add_file(params.body.clone()).await;
            let stikipage = StikiPage {
                body: params.body.clone(),
                refs: params.refs.clone(),
            };
            let stiki_wiki = ipfs
                .add_file(serde_json::to_string(&stikipage).unwrap())
                .await;
            Ok(HttpResponse::Ok().content_type("application/json").body(
                serde_json::to_string(&NewStikiResp {
                    status: StatusEnum::OK.as_str().to_owned(),
                    message: Some("Success".to_owned()),
                    body: Some(stiki_wiki),
                })
                .unwrap(),
            ))
        }
        _ => Ok(HttpResponse::Gone().content_type("application/json").body(
            serde_json::to_string(&NewStikiResp {
                status: StatusEnum::ERR.as_str().to_owned(),
                message: Some("Page could not be added.".to_owned()),
                body: None,
            })
            .unwrap(),
        )),
    }
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
async fn get_stiki(stikiid: web::Path<String>) -> impl Responder {
    match IPFS.get() {
        Some(ipfs) => ipfs.get_stiki(stikiid.to_string()).await,
        _ => "".to_owned(),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let ipfs = IpfsInterface::new().await;
    match IPFS.set(ipfs) {
        Ok(_) => println!("OK"),
        _ => println!("Nok"),
    };
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://0.0.0.0:*")
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
