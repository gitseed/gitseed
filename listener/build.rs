fn main() {
    pkg_config::Config::new().probe("uuid").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
