use std::fs::File;
use std::io::Read;
use std::sync::Once;

const FILE_PATH: &str = "configs/dev.toml";

static mut CONFIG: Option<&'static Config> = None;
static CONFIG_SET: Once = Once::new();

#[derive(serde::Deserialize)]
pub struct Config {
    pub application: Application,
}

#[derive(serde::Deserialize)]
pub struct Application {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(serde::Deserialize)]
pub struct Database {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(serde::Deserialize)]
pub struct OauthGithub {
    pub client_id: String,
    pub client_secret: String,
}

fn read_config_file() -> Result<Config, std::io::Error> {
    let mut s = String::new();
    File::open(FILE_PATH)?.read_to_string(&mut s)?;
    Ok(toml::from_str(&s).unwrap())
}

fn init_config() -> Option<&'static Config> {
    CONFIG_SET.call_once(|| {
        let c = Box::new(match read_config_file() {
            Ok(c) => c,
            Err(e) => panic!("config reading failed: {:?}", e),
        });
        unsafe {
            CONFIG = Some(Box::leak(c));
        }
    });
    unsafe { CONFIG }
}

pub fn config() -> &'static Config {
    if CONFIG_SET.is_completed() {
        unsafe { CONFIG.unwrap() }
    } else {
        init_config().unwrap()
    }
}
