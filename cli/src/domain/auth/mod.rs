use std::{
    fs::{remove_file, File},
    io::{BufReader, BufWriter},
};

use anyhow::{bail, Context};

mod file;
use base64::{
    alphabet::URL_SAFE,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};
use console::{style, Emoji};
use file::AuthFile;

mod claims;
use claims::{Claims, ExtraClaims};

use super::Env;

pub struct AuthContext {
    token: String,
    pub claims: Claims<ExtraClaims>,
}

const ENGINE_CONFIG: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::RequireNone)
    .with_decode_allow_trailing_bits(true);

const ENGINE: GeneralPurpose = GeneralPurpose::new(&URL_SAFE, ENGINE_CONFIG);

const AUTH_FILE: &str = "auth.json";

static WARNING: Emoji<'_, '_> = Emoji("⚠️ ", "");

impl AuthContext {
    pub fn token(&self) -> &String {
        &self.token
    }
    pub fn new_from_token(token: String) -> anyhow::Result<Self> {
        let mut parts = token.split('.');

        let _header = parts.next().context("Invalid token: no header")?;
        let mut payload = parts.next().context("Invalid token")?.as_bytes();
        let payload_r = base64::read::DecoderReader::new(&mut payload, &ENGINE);
        let claims: Claims<ExtraClaims> = serde_json::from_reader(payload_r)?;

        Ok(Self { token, claims })
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

        Ok(Some(Self::new_from_token(auth_file.token)?))
    }

    pub fn auth_cached(env: &Env) -> anyhow::Result<Self> {
        if let Some(auth) = Self::from_env()? {
            Ok(auth)
        } else {
            env.println(format!(
                "{} {}You must be authentificated, run `swarmd login`!",
                style("").red().bold().dim(),
                WARNING
            ))?;
            bail!("You must be logged in.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_snapshot() {
        let token_test = "no_header.eyJjbGVya191c2VyX2lkIjoidXNlcl8yWG82c0dQU0JmMlVnS0hOeWRZMnBqVFNncXkiLCJjbGkiOnRydWUsImN1cnJlbnRfb3JnX2NsZXJrX2lkIjpudWxsLCJkZWZhdWx0X29yZyI6Im9yZ18yWG83TDRjbHZPSEp2a0hjSjZ1Y29JTE5JNkMiLCJleHAiOjE2OTk2OTk4ODgsImlhdCI6MTY5OTYxMzQ4OCwiaXNzIjoiaHR0cHM6Ly9ub3JtYWwta2lkLTM4LmNsZXJrLmFjY291bnRzLmRldiIsImp0aSI6ImQ2NTA1ZjkwYTk4OWUwMjg4ODE2IiwibmJmIjoxNjk5NjEzNDgzLCJvcmdfc2x1ZyI6bnVsbCwib3JncyI6eyJvcmdfMlhvN0w0Y2x2T0hKdmtIY0o2dWNvSUxOSTZDIjoiYWRtaW4ifSwic3ViIjoidXNlcl8yWG82c0dQU0JmMlVnS0hOeWRZMnBqVFNncXkiLCJ1c2VyX2lkIjpudWxsfQ.no_signin";

        let mut parts = token_test.split('.');

        let _header = parts.next().context("Invalid token: no header").unwrap();
        let mut payload = parts.next().context("Invalid token").unwrap().as_bytes();
        let payload_r = base64::read::DecoderReader::new(&mut payload, &ENGINE);
        let claims: Claims<ExtraClaims> = serde_json::from_reader(payload_r).unwrap();
        insta::assert_yaml_snapshot!(claims);
    }
}
