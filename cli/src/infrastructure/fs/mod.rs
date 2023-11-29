use anyhow::Context;
use directories::ProjectDirs;

pub fn base_directory() -> anyhow::Result<ProjectDirs> {
    let conf =
        ProjectDirs::from("io.swarmd", "swarmd", "cli").context("Couln't generate project dir.")?;
    Ok(conf)
}
