use std::env;

pub fn init_logger() {
    let rust_log = "RUST_LOG";
    match env::var(rust_log) {
        Ok(value) => {
            println!("RUST_LOG level {}", value);
        }
        Err(_) => {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::builder().init();
}