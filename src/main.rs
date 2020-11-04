mod back;

use {
    std::process::{Command, Stdio},
    back::device::Device,
    std::io::{stdin},
    back::connect::connect_device,
};

fn main() {
    let result = run();
    match result {
        Ok(_) => {}
        Err(value) => {
            println!("{}", value)
        }
    }
}

fn run() -> Result<bool, String> {
    if !is_adb_installed() {
        return Err(String::from("ADB is not installed"))
    }
    let device_list = Device::get_list_of_devices();
    if device_list.len() == 0 {
        return Err(String::from("- Error: No device is connected\n\nHint: Check your USB cable & see if USB Debugging option is enabled."));
    }

    println!("Finding appropriate device...");

    let device: &Device = if device_list.len() > 1 {
        choose_a_device(&device_list).unwrap()
    } else {
        device_list.first().unwrap()
    };
    connect_device(device);
    return Ok(true)
}

fn choose_a_device(v: &Vec<Device>) -> Option<&Device> {
    for (i,x) in v.iter().enumerate() {
        println!("{}. {}", i+1, x.device_id);
    }
    println!("Choose a device: ");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read input.");
    let input: usize = buffer.trim().parse().expect("Enter a number not a string.");
    return Some(&v[input-1]);
}

fn is_adb_installed() -> bool {
    Command::new("adb").arg("--version").stdout(Stdio::null()).status().is_ok()
}