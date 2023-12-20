use std::{path::PathBuf, process::Output};

use config::Config;
use console::{style, Emoji};
use serde::{Deserialize, Serialize};
use swarmd_generated::models::{
    CreateProjectPayload, CreateProjectResponse, GetBySlugResponse, PublishWorkerPayload,
    PublishWorkerResponse, UploadWorkerResponse,
};

use crate::package::npm_rs::{NodeEnv, NpmEnv};
use swarmd_slug_rs::Slug;

use super::{auth::AuthContext, Env};

static CREATE: Emoji<'_, '_> = Emoji("🏗 ", "");
pub const SWARMD_CONFIG_FILE: &str = "swarmd.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProfileEnum {
    #[serde(rename = "dev")]
    Dev(ProfileConfig),
    #[serde(rename = "prod")]
    Prod(ProfileConfig),
}

impl ProfileEnum {
    fn to_node_env(&self) -> NodeEnv {
        match &self {
            ProfileEnum::Dev(_) => NodeEnv::Development,
            ProfileEnum::Prod(_) => NodeEnv::Production,
        }
    }

    fn get_config(&self) -> &ProfileConfig {
        match &self {
            ProfileEnum::Dev(config) | ProfileEnum::Prod(config) => config,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileConfig {
    /// Build command to run
    build_command: String,
    /// Worker main file
    worker_main: PathBuf,
}

/// Configuration file for the application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkerConfig {
    /// Define the account linked to this Project or it'll use the default one
    organization_id: Option<String>,
    /// The slug of the associated project.
    /// Must be < 14
    pub name: Slug,
    profile: ProfileEnum,
}

impl WorkerConfig {
    /// Read the associated WorkerConfig
    pub fn from_file(file_name_to_locate: &str) -> anyhow::Result<Self> {
        let workerconfig = Config::builder()
            .add_source(config::File::with_name(file_name_to_locate))
            .add_source(
                config::Environment::with_prefix("SWARMD")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()?
            .try_deserialize::<Self>()?;

        Ok(workerconfig)
    }

    pub fn execute_build(&self) -> anyhow::Result<()> {
        let output = self.execute_no_log()?;
        let exit_status = output.status;

        if exit_status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Couldn't execute the build_command properly"
            ))
        }
    }

    pub fn execute_no_log(&self) -> anyhow::Result<Output> {
        let profile = &self.profile;
        let config = profile.get_config();
        Ok(NpmEnv::default()
            .with_node_env(&profile.to_node_env())
            .init_env()
            .raw_append(&config.build_command)
            .exec_no_log()?)
    }

    pub fn path_main_dist(&self) -> &PathBuf {
        let profile = &self.profile;
        let config = profile.get_config();
        &config.worker_main
    }

    pub fn read_dist_sync(&self) -> anyhow::Result<Vec<u8>> {
        let profile = &self.profile;
        let config = profile.get_config();
        let path = &config.worker_main;

        let file = std::fs::read(path)?;
        Ok(file)
    }

    /// Get the OrganizationID from the Config file or get it from the AuthContext
    pub fn organization_id(&self, auth: &AuthContext) -> String {
        self.organization_id
            .clone()
            .unwrap_or_else(|| auth.claims.extra.default_org.clone())
    }

    /// Create the associated project if it doesn't exist yet.
    pub async fn create_project_associated(
        &self,
        env: &Env,
        auth: &AuthContext,
    ) -> anyhow::Result<String> {
        let org_id = self.organization_id(auth);
        let project_slug = &self.name;

        let swarmd_client = env.swarmd_client()?;

        // Get project_id or Create a new project
        let GetBySlugResponse { project } =
            swarmd_generated::apis::project_api::organization_id_project_slug_project_slug_get(
                swarmd_client.as_ref(),
                &org_id,
                project_slug.as_ref(),
            )
            .await?;

        if let Some(id) = project.flatten().and_then(|x| x.id).flatten() {
            return Ok(id);
        }

        let CreateProjectResponse { id } =
            swarmd_generated::apis::project_api::organization_id_project_create_post(
                swarmd_client.as_ref(),
                &org_id,
                CreateProjectPayload {
                    slug: project_slug.to_string(),
                },
            )
            .await?;

        env.println(format!(
            "{} {} Project {} created as it didn't exist.",
            style("").bold().dim(),
            CREATE,
            style(project_slug).bold().cyan(),
        ))?;

        Ok(id)
    }

    /// Upload a worker which will create the associated project if it doesn't exist yet.
    pub async fn upload_worker(
        &self,
        env: &Env,
        auth: &AuthContext,
        project_id: &str,
        body: Vec<u8>,
    ) -> anyhow::Result<String> {
        let org_id = self.organization_id(auth);

        let swarmd_client = env.swarmd_client()?;

        let UploadWorkerResponse { worker_id } = swarmd_generated::apis::worker_api::organization_id_project_project_id_worker_upload_post(swarmd_client.as_ref(), &org_id, project_id, body).await?;

        Ok(worker_id)
    }

    /// Deploy a worker
    pub async fn deploy_worker(
        &self,
        env: &Env,
        auth: &AuthContext,
        project_id: &str,
        deployed_worker_id: String,
    ) -> anyhow::Result<Option<String>> {
        let org_id = self.organization_id(auth);

        let swarmd_client = env.swarmd_client()?;

        let PublishWorkerResponse { id: _, route } =
            swarmd_generated::apis::project_api::organization_id_project_project_id_publish_put(
                swarmd_client.as_ref(),
                &org_id,
                project_id,
                PublishWorkerPayload {
                    worker_id: deployed_worker_id,
                },
            )
            .await?;

        // Launch publish
        Ok(route)
    }
}
