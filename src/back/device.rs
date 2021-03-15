use std::process::Command;
use regex::Regex;
use std::io::stdin;
use std::process::exit;

#[derive(Clone, Copy)]
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

    const DEVICE_NOT_CONNECTED: &'static str = "- Error: No device is connected\n\nHint: If devices are connected but not visible then check your USB cable & see if USB Debugging option is enabled.";

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

    pub fn get_list_of_connected_device() -> Option<Vec<Device>> {
        let re = Regex::new(r"[.]").unwrap();
        let mut v: Vec<Device> = Vec::new();

        let x = Device::get_list_of_devices();
        for c in x {
            if re.is_match(&c.device_id.to_owned()) {
                v.push(c);
            }
        }

        if v.len() > 0 {
            return Some(v)
        }

        None
    }

    pub fn choose_a_device(v: &Vec<Device>) -> Option<&Device> {
        for (i,x) in v.iter().enumerate() {
            println!("{}. {}", i+1, x.device_id);
        }
        println!("Choose a device: ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read input.");
        let input: usize = match buffer.trim().parse() {
            Ok(v) => { v }
            Err(_) => { 0 }
        };
        if input == 0 {
            println!("Error: Index cannot be 0 or unknown!");
            exit(1)
        }
        return Some(&v[input-1])
    }

    pub fn get_or_choose_connected_device() -> Option<Device> {
        let device_list = Device::get_list_of_devices();
        if device_list.len() == 0 {
            println!("{}", Device::DEVICE_NOT_CONNECTED);
            return None
        }

        let device: &Device  = if device_list.len() > 1 {
            Device::choose_a_device(&device_list).unwrap()
        } else {
            device_list.first().unwrap()
        };

        let n: Device = Device { device_id: device.device_id.to_owned(), status: device.status.to_owned() };

        return Some(n);
    }
}