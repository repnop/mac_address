fn main() {
    match std::env::var("TARGET") {
        Ok(ref t) if t.contains("windows") => println!("cargo:rustc-link-lib=iphlpapi"),
        _ => {}
    }
}
