use xdg::BaseDirectories;

pub fn base_directory() -> anyhow::Result<BaseDirectories> {
    let conf = BaseDirectories::with_prefix("swarmd")?;
    Ok(conf)
}
