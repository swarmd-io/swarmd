use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::{collections::HashMap, time::Duration};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    /// Single value
    One(T),
    /// Array of values
    Vec(Vec<T>),
}

impl<T> OneOrMany<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        matches!(self, OneOrMany::Vec(v) if v.is_empty())
    }
}

impl<T> Default for OneOrMany<T> {
    #[inline]
    fn default() -> Self {
        Self::Vec(Vec::new())
    }
}

#[serde_as]
#[skip_serializing_none]
#[non_exhaustive]
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct Claims<T> {
    #[serde_as(as = "Option<serde_with::DurationSeconds<f64>>")]
    pub exp: Option<Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<f64>>")]
    pub nbf: Option<Duration>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<f64>>")]
    pub iat: Option<Duration>,

    pub iss: Option<String>,
    pub sub: Option<String>,
    #[serde(default, skip_serializing_if = "OneOrMany::is_empty")]
    pub aud: OneOrMany<String>,
    pub jti: Option<String>,

    #[serde(flatten)]
    pub extra: T,
}

// Should be in sync with JWT template used for CLI
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraClaims {
    clerk_user_id: String,
    cli: bool,
    current_org_clerk_id: Option<String>,
    pub default_org: String,
    org_slug: Option<String>,
    // Org_id -> Role
    orgs: HashMap<String, String>,
}
