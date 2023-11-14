use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(target_family = "windows")] {
        const CMD: &str = "cmd.exe";
        const OPT: &str = "/C";
    } else {
        const CMD: &str = "bash";
        const OPT: &str = "-c";
    }
}
