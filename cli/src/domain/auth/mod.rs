use std::{
    fs::{remove_file, File},
    io::{BufReader, BufWriter, Write},
};

use anyhow::Context;

mod file;
use file::AuthFile;

pub struct AuthContext {
    token: String,
}

const AUTH_FILE: &str = "auth.json";

impl AuthContext {
    pub fn new_from_token(token: String) -> Self {
        Self { token }
    }

    pub fn save(self) -> anyhow::Result<()> {
        let base =
            crate::infrastructure::fs::base_directory().context("Couln't load base directory")?;

        let path = base.place_state_file(AUTH_FILE)?;
        let auth_file = File::create(path)?;
        let writer = BufWriter::new(auth_file);
        let auth = AuthFile { token: self.token };
        serde_json::to_writer_pretty(writer, &auth)?;
        Ok(())
    }

    /// Load AuthContext from environment by loading the associated token through the associated
    /// file.
    pub fn from_env() -> anyhow::Result<Option<Self>> {
        let base =
            crate::infrastructure::fs::base_directory().context("Couln't load base directory")?;

        let file_path = match base.find_state_file(AUTH_FILE) {
            None => return Ok(None),
            Some(file) => file,
        };

        let file_opened = File::open(file_path.clone()).context("Couldn't open file")?;
        let reader = BufReader::new(file_opened);
        let auth_file: AuthFile = match serde_json::from_reader(reader) {
            Err(err) => {
                tracing::error!("{err:?}");
                // If the file is corrupted, we delete it.
                remove_file(file_path).context("Coudln't delete auth file")?;
                return Ok(None);
            }
            Ok(elt) => elt,
        };

        Ok(Some(Self {
            token: auth_file.token,
        }))
    }
}
