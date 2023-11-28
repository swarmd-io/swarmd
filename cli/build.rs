use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let date = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .unwrap();
    let date = String::from_utf8(date.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_DATE={}", date);
}
