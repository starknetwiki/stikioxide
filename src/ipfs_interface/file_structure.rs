use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StikiPage {
    pub(crate) body: String,
    pub(crate) refs: HashMap<String, String>,
}
