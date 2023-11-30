use self_update::update::Release;
use tokio::task::spawn_blocking;

pub async fn get_last_release() -> anyhow::Result<Option<Release>> {
    let release = spawn_blocking(|| {
        let release = self_update::backends::github::Update::configure()
            .repo_owner("swarmd-io")
            .repo_name("swarmd")
            .bin_name("swarmd")
            .current_version(self_update::cargo_crate_version!())
            .build()?
            .get_latest_release()?;

        Ok::<_, anyhow::Error>(release)
    })
    .await?
    .ok();

    Ok(release)
}

pub async fn check_if_update_available() -> anyhow::Result<Option<String>> {
    if let Some(release) = get_last_release().await? {
        let current_version = self_update::cargo_crate_version!();
        if current_version == release.version {
            Ok(None)
        } else {
            Ok(Some(release.version))
        }
    } else {
        Ok(None)
    }
}
