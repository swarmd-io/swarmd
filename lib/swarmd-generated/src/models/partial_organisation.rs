/*
 * Swarmd Backend API Documentation for swarmd_api version 0.1.0
 *
 * # Introduction  blblbllb
 *
 * The version of the OpenAPI document:
 * Contact: anthony@brevz.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialOrganisation {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "role")]
    pub role: crate::models::OrgRole,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl PartialOrganisation {
    pub fn new(id: String, role: crate::models::OrgRole, slug: String) -> PartialOrganisation {
        PartialOrganisation { id, role, slug }
    }
}