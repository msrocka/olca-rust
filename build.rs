use std::fs;

#[cfg(target_os = "macos")]
static OS: &str = "macos";
#[cfg(target_os = "linux")]
static OS: &str = "linux";
#[cfg(target_os = "windows")]
static OS: &str = "windows";

fn main() {

    // when we compile the library we link against the
    // math libraries provided by the Julia distribution.
    // we read the path of the library folder from the
    // config.toml file. if we cannot find it, we take
    // the ./bin folder as library path

    let libdir_key: String = format!("{}-julia-lib-dir", OS);
    let libdir_key: &str = libdir_key.as_str();
    let mut libdir: Option<&str> = None;

    let config = fs::read_to_string("config")
        .expect("Could not read config file");
    for line in config.lines() {
        let l: &str = line.trim();
        if !l.starts_with(libdir_key) {
            continue;
        }
        let parts: Vec<&str> = l.split("=").collect();
        if parts.len() < 2 {
            continue;
        }
        let path = parts[1].trim();
        libdir = Some(path);
    }

    println!("cargo:rustc-link-search={}", libdir.unwrap_or("./bin"));
}
