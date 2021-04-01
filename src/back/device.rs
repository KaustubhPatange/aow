use std::process::Command;
use regex::Regex;
use std::io::stdin;

#[derive(Clone, Copy)]
pub enum Status {
    ONLINE,
    OFFLINE,
    UNAUTHORIZED,
}

#[derive(Clone)]
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
            println!("- Error: Index cannot be 0 or unknown!");
            return None
        }
        return Some(&v[input-1]);
    }

    pub fn get_or_choose_connected_device() -> Option<Device> {
        let device_list = Device::get_list_of_devices();
        if device_list.len() == 0 {
            println!("- Error: No device is connected\n\nHint: If devices are connected but not visible then check your USB cable & see if USB Debugging option is enabled.");
            return None
        }

        let device: &Device  = if device_list.len() > 1 {
            match Device::choose_a_device(&device_list) {
                None => {
                    return None
                }
                Some(device) => {
                    device
                }
            }
        } else {
            device_list.first().unwrap()
        };

        // This guarantees that device with "Online" status are sent back.
        return match device.status {
            Status::ONLINE => {
                Some(device.clone())
            }
            Status::OFFLINE => {
                Device::print_device_offline(device.device_id.as_str());
                None
            }
            Status::UNAUTHORIZED => {
                Device::print_device_unauthorized(device.device_id.as_str());
                None
            }
        };
    }

    fn print_device_unauthorized(device_id: &str) {
        println!("- Error: Device {} is unauthorized", device_id);
        println!();
        println!("Hint: Accept the prompt in your device & re run the command.")
    }

    fn print_device_offline(device_id: &str) {
        println!("- Error: Device {} is offline", device_id);
        println!();
        println!("Hint: Try disconnecting & re-connecting device or use aow -d to disconnect from all devices.")
    }
}