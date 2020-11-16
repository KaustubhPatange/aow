/**
* A module created to manage app's internet settings.
*/

use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::{
    fs::{File},
    env::temp_dir,
    path::{PathBuf, Path},
    io::{Write, Read},
};

const DATE_NO_DASH_FORMAT: &str = "%Y%m%d";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub last_checked: String,
}

impl Config {
    pub fn need_to_check_update() -> bool {
        let mut config = Config::read();

        if config.last_checked != "" {
            let today = Utc::now().format(DATE_NO_DASH_FORMAT).to_string().parse::<i32>().unwrap();
            let old = config.last_checked.parse::<i32>().unwrap();

            if today <= old { return false; }
        }

        let next: String = Utc::now()
            .checked_add_signed(Duration::days(1))
            .unwrap()
            .format(DATE_NO_DASH_FORMAT).to_string();

        config.last_checked = next;

        Config::write(&config);

        return true
    }

    fn exist() -> bool {
        Path::new(Config::get_path().to_str().unwrap()).exists()
    }

    fn read() -> Config {
        if !Config::exist() {
            let d = Config::get_default();
            Config::write(&d);
            return d;
        }
        let mut file = File::open(Config::get_path()).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).ok();

        let config: Config = serde_json::from_str(buffer.as_str()).unwrap();

        return config;
    }

    fn write(c: &Config) {
        let mut file = File::create(Config::get_path()).unwrap();
        let j = serde_json::to_string(&c).unwrap();
        file.write(j.as_ref()).ok();
    }

    fn get_path() -> PathBuf {
        return temp_dir().join(".aow");
    }

    fn get_default() -> Config {
        return Config {
            last_checked: String::from("")
        }
    }
}