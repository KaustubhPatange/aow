use std::process::Command;
use regex::Regex;

pub enum Status {
    ONLINE,
    OFFLINE,
    UNAUTHORIZED,
}

pub struct Device {
    pub device_id: String,
    pub status: Status,
}

impl Device {
    pub fn get_list_of_devices() -> Vec<Device> {
        let output = Command::new("adb").arg("devices").output().expect("");
        let output = String::from_utf8_lossy(&output.stdout);

        let re = Regex::new(r"[\s]+").unwrap();

        return output.lines()
            .filter(|x| !x.starts_with("*") && !x.is_empty() && !x.contains("List of devices attached") && !x.starts_with("emulator"))
            .map(|x| {
                let val = re.replace(x, ",").to_string();
                let val = val.split(",").collect::<Vec<_>>();
                let status = match val[1] {
                    "offline" => Status::OFFLINE,
                    "unauthorized" => Status::UNAUTHORIZED,
                    _ => Status::ONLINE,
                };
                Device { device_id: String::from(val[0]), status }
            })
            .collect::<Vec<_>>();
    }
}