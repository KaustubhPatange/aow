mod back;

use {
    std::process::{Command, Stdio},
    back::device::Device,
    std::io::{stdin},
    std::env,
    back::{
        connect::connect_device,
        command::parse_command,
        update::fetch_new_version,
    },
};

#[tokio::main]
async fn main() {
    if !is_adb_installed() {
        println!("ADB is not installed");
        return
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        parse_command(&args);
        return
    }
    let result = run();
    match result {
        Ok(_) => {}
        Err(value) => {
            println!("{}", value)
        }
    }

    fetch_new_version().await;
}

fn run() -> Result<bool, String> {
    let device_list = Device::get_list_of_devices();
    if device_list.len() == 0 {
        return Err(String::from("- Error: No device is connected\n\nHint: If devices are connected but not visible then check your USB cable & see if USB Debugging option is enabled."));
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