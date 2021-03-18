mod back;

use {
    std::process::{Command, Stdio},
    std::env,
    back::{
        device::Device,
        connect::connect_device,
        command::parse_command,
        update::fetch_new_version,
    },
};
use std::borrow::Borrow;

#[tokio::main]
async fn main() {
    if !is_adb_installed() {
        println!("ADB is not installed");
        return
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        parse_command(&args);
    } else {
        let result = run();
        match result {
            Ok(_) => {}
            Err(_) => {
                //error will be printed
            }
        }
    }
    fetch_new_version().await;
}

fn run() -> Result<bool, ()> {
    let result: Option<Device> = Device::get_or_choose_connected_device();
    return match result {
        Some(device) => {
            connect_device(device.borrow());
            Ok(true)
        }
        None => {
            Err(())
        }
    }
}

fn is_adb_installed() -> bool {
    Command::new("adb").arg("--version").stdout(Stdio::null()).status().is_ok()
}