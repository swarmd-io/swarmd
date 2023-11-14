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
pub struct Project {
    #[serde(rename = "active_worker", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_worker: Option<Option<String>>,
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "org_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Option<String>>,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl Project {
    pub fn new(slug: String) -> Project {
        Project {
            active_worker: None,
            id: None,
            org_id: None,
            slug,
        }
    }
}


