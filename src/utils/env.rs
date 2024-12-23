use std::env;

pub fn get_env_var(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}