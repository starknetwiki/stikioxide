use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Stiki {
    pub body: String,
    pub refs: HashMap<String, String>,
    pub id: Option<String>,
    pub pinned: Option<bool>,
}

pub(crate) enum StatusEnum {
    OK,
    ERR,
}

impl StatusEnum {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            StatusEnum::OK => "OK",
            StatusEnum::ERR => "ERR",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewStikiResp {
    pub status: String,
    pub message: Option<String>,
    pub body: Option<String>,
}
