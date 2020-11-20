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
    pub new_version: String,
    pub update_needed: bool,
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

        return true;
    }

    pub fn set_update_needed(value: bool) {
        let mut config = Config::read();
        config.update_needed = value;

        Config::write(&config);
    }

    pub fn is_update_needed() -> bool {
        return Config::read().update_needed;
    }

    pub fn set_new_version(value: String) {
        let mut config = Config::read();
        config.new_version = value;

        Config::write(&config);
    }

    pub fn get_new_version() -> String {
        return Config::read().new_version;
    }

    fn exist() -> bool {
        Path::new(Config::get_path().to_str().unwrap()).exists()
    }

    fn read() -> Config {
        if !Config::exist() {
            Config::write_default();
            return Config::get_default();
        }
        let mut file = File::open(Config::get_path()).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).ok();

        let config = serde_json::from_str(buffer.as_str());
        return match config {
            Ok(value) => {
                value
            }
            Err(_) => {
                Config::write_default();
                Config::get_default()
            }
        }
    }

    fn write(c: &Config) {
        let mut file = File::create(Config::get_path()).unwrap();
        let j = serde_json::to_string(&c).unwrap();
        file.write(j.as_ref()).ok();
    }

    fn get_path() -> PathBuf {
        return temp_dir().join(".aow");
    }

    fn write_default() {
        let d = Config::get_default();
        Config::write(&d);
    }

    fn get_default() -> Config {
        return Config {
            last_checked: String::from(""),
            new_version: String::from(""),
            update_needed: false,
        };
    }
}