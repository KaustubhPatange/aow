use crate::back::device::Device;
use crate::back::connect::disconnect;
use std::process::{Command, exit};
use std::path::Path;

#[cfg(target_os = "windows")]
extern crate wfd;
#[cfg(target_os = "windows")]
use self::wfd::{DialogError, DialogParams};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn parse_command(c: &Vec<String>) {
    if c.contains(&"-h".to_owned()) || c.contains(&"--help".to_owned()) {
        print_all_commands()
    } else if c[1].eq("-s") || c[1].eq("--show") {
        let _ = show_connected_device();
    } else if c[1].eq("-v") || c[1].eq("--version") {
        println!("{}", VERSION)
    } else if c[1].eq("snap") { // snap
        if c.len() == 3 {
            take_snap(c[2].as_str());
        } else {
            take_snap("");
        }
    } else if c[1].eq("dlk") || c[1].eq("deeplink") { // deeplink
        if c.len() == 3 {
            deeplink(c[2].as_str())
        } else {
            println!("- Error: No url attached to the command")
        }
    } else if c[1].eq("-d") || c[1].eq("--disconnect") {
        let result = show_connected_device();
        match result {
            Ok(_) => {
                disconnect();
            }
            Err(_) => {}
        }
    }
}

fn show_connected_device() -> Result<Vec<Device>, bool> {
    let d = Device::get_list_of_connected_device();
    match d {
        Some(value) => {
            for v in value.iter() {
                println!("- Found {} as connected device", v.device_id);
            }
            Ok(value)
        }
        None => {
            println!("- Error: No device is connected over wifi");
            Err(false)
        }
    }
}

fn take_snap(file_path: &str) {
    let result = Device::get_or_choose_connected_device();
    match result {
        Some(device) => {
            Command::new("adb").arg("-s").arg(device.device_id.as_str()).arg("shell").arg("screencap").arg("-p").arg("/data/local/tmp/file.png").spawn().unwrap().wait().ok();
            let save_path: String = if file_path != "" {
                String::from(file_path)
            } else {
                // show save dialog for windows only
                if cfg!(target_os="windows") {
                    let params = DialogParams {
                        file_name: "file.png",
                        file_types: vec![("png", "*.png")],
                        title: "Choose a path to save",
                        ..Default::default()
                    };
                    let result = wfd::save_dialog(params);
                    let path: String = match result {
                        Ok(file) => {
                            String::from(file.selected_file_path.to_str().unwrap())
                        }
                        Err(e) => {
                            match e {
                                DialogError::HResultFailed { hresult, error_method} => {
                                    println!("- Error: HResult Failed - HRESULT: {:X}, Method: {}", hresult, error_method);
                                }
                                DialogError::UnsupportedFilepath => { println!("- Error: Unsupported file path"); }
                                DialogError::UserCancelled => { }
                            }
                            exit(1);
                        }
                    };
                    path
                } else {
                    println!("- Error: Native dialogs are not supported on {}", std::env::consts::OS);
                    exit(1);
                }
            };
            Command::new("adb").arg("-s").arg(device.device_id.as_str()).arg("pull").arg("/data/local/tmp/file.png").arg(save_path.as_str()).spawn().unwrap().wait().ok();
            if Path::new(save_path.as_str()).exists() {
                println!("Saved to: {}", save_path);
            } else {
                println!("Unknown error while saving file");
            }
        }
        None => {}
    }
}

fn deeplink(link: &str) {
    match Device::get_or_choose_connected_device() {
        None => {}
        Some(device) => {
            println!("Launching => {}", link);
            Command::new("adb").args(&["-s", device.device_id.as_str(), "shell", "am", "start", "-d", link]).spawn().unwrap().wait().ok();
        }
    }
}

fn print_all_commands() {
    println!("ADB Over Wifi (aow) v{} - A command line tool to connect devices over wifi (requires ADB).", VERSION);
    println!("Copyright 2020 Kaustubh Patange - https://github.com/KaustubhPatange/aow");
    println!();
    println!("Usage: aow [options]");
    println!();
    println!("Options:");
    println!("      [null]               Connects a device over wifi (see demo on Github)");
    println!("      -s, --show           Shows the list of connected device over wifi (if any).");
    println!("      -d, --disconnect     Disconnect the connected device (if any).");
    println!("      -v, --version        Prints the current version of tool");
    println!("      -h, --help           Prints this help message.");
    println!("      snap [file-path]     Take a screenshot. Optionally, you can specify a path to save it otherwise");
    println!("                           the program will open native save dialog (windows only) to save the file.");
    println!("      dlk, deeplink [url]  Fires a deeplink with the \"url\".");
    println!();
    println!("Examples:");
    println!("      aow");
    println!("      aow -d");
}