use std::fs;
use std::sync::OnceLock;
use toml::Value;

static CONFIG: OnceLock<Option<Value>> = OnceLock::new();

fn load_config() -> Option<Value> {
    let contents = fs::read_to_string("config.toml").ok()?;
    toml::from_str(&contents).ok()
}

fn get_config() -> &'static Option<Value> {
    CONFIG.get_or_init(load_config)
}

pub fn get(key: &str) -> Option<&'static Value> {
    let config = get_config().as_ref()?;
    config.get(key)
}
