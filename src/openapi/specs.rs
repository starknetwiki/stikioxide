use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Stiki {
    pub body: String,
    pub refs: HashMap<String, String>,
    pub id: Option<String>,
    pub pinned: bool,
}

pub(crate) enum StatusEnum {
    OK,
    ERR,
}

impl StatusEnum {
    fn as_str(&self) -> &'static str {
        match self {
            StatusEnum::OK => "OK",
            StatusEnum::ERR => "ERR",
        }
    }
}

pub(crate) struct NewStikiResp {
    status: String,
    message: Option<String>,
    body: Option<String>,
}
