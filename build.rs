fn main() {
    use std::env;

    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if os == "macos" {
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
    }
}
