use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub short: String,
    pub long: String,
}