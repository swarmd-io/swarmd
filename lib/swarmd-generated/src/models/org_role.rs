/*
 * Swarmd Backend API Documentation for swarmd_api version 0.1.0
 *
 * # Introduction  blblbllb
 *
 * The version of the OpenAPI document:
 * Contact: anthony@brevz.io
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrgRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
}

impl ToString for OrgRole {
    fn to_string(&self) -> String {
        match self {
            Self::Admin => String::from("admin"),
            Self::Member => String::from("member"),
        }
    }
}

impl Default for OrgRole {
    fn default() -> OrgRole {
        Self::Admin
    }
}
