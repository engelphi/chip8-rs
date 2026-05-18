use std::env;
use std::process::Command;

fn main() {
    println!("{}", env::var("CRATE_TARGET").unwrap());
    let profile = env::var("CRATE_PROFILE").unwrap();
    let lib_name = "libchip8_cxx.a";
    let target_dir = std::path::PathBuf::from(env::var("CRATE_TARGET_DIR").unwrap());
    let output_dir = target_dir
        .join("universal-apple-darwin")
        .join(profile.clone());
    std::fs::create_dir_all(&output_dir).unwrap();
    let status = Command::new("lipo")
        .arg("-create")
        .arg("-output")
        .arg(output_dir.join(lib_name).to_str().unwrap())
        .arg(
            target_dir
                .join("aarch64-apple-darwin")
                .join(profile.clone())
                .join(lib_name)
                .to_str()
                .unwrap(),
        )
        .arg(
            target_dir
                .join("x86_64-apple-darwin")
                .join(profile.clone())
                .join(lib_name)
                .to_str()
                .unwrap(),
        )
        .status()
        .expect("Failed to build project");
    if !status.success() {
        panic!("Failed to build the project");
    }
}
