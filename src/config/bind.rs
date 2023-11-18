use super::fields::Config;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::sync::Once;

const ENV_CONFIG_FILE: &str = "CONFIG_FILE";

static mut CONFIG: Option<&'static Config> = None;
static CONFIG_SET: Once = Once::new();

fn read_config_file() -> Result<Config, Box<dyn Error>> {
    let mut str = String::new();
    let file = env::var(ENV_CONFIG_FILE)?;
    File::open(file)?.read_to_string(&mut str)?;
    let cfg = toml::from_str(&str)?;
    Ok(cfg)
}

fn init_global_config() -> Option<&'static Config> {
    CONFIG_SET.call_once(|| {
        let c = Box::new(match read_config_file() {
            Ok(c) => c,
            Err(e) => panic!("failed to read config file: {:?}", e),
        });
        unsafe {
            CONFIG = Some(Box::leak(c));
        }
    });
    unsafe { CONFIG }
}

pub fn global_config() -> &'static Config {
    if CONFIG_SET.is_completed() {
        unsafe { CONFIG.unwrap() }
    } else {
        init_global_config().expect("failed to init config")
    }
}
