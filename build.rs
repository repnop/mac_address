use std::env;

fn main() {
    let target = match env::var("TARGET") {
        Err(e) => {eprintln!("Err:Can't get env variable. {:?}", e); "NO TARGET".to_string()},
        Ok(val) => val,
    };
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=iphlpapi");
    }
}