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
    let build_date = env!("BUILD_DATE");
    let now = chrono::Utc::now();
    let date = chrono::DateTime::parse_from_rfc3339(build_date)?;
    let dur = now.signed_duration_since(date);

    // We only check after 3 days
    // TODO: do only one check a day and store the data to reuse until tomorrow.
    if dur < chrono::Duration::days(3) {
        return Ok(None);
    }

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
