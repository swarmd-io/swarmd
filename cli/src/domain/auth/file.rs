use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthFile {
    pub token: String,
}
