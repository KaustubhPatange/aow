use crate::back::device::Device;
use crate::back::connect::disconnect;
use std::process::{Command};
use std::path::Path;
use crate::back::dialog::launch_windows_save_dialog;
use directories::UserDirs;
use std::time::Duration;
use std::io::{Write};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const NULL: &'static str = "null";

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
    } else if c[1].eq("rec") {
        if c.len() == 3 {
            screen_record(c[2].as_str())
        } else {
            screen_record(NULL);
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
                let file = match launch_windows_save_dialog() {
                    Ok(file_path) => {
                        file_path
                    }
                    _ => {
                        NULL.to_owned()
                    }
                };
                file
            };
            if save_path != NULL {
                Command::new("adb").arg("-s").arg(device.device_id.as_str()).arg("pull").arg("/data/local/tmp/file.png").arg(save_path.as_str()).spawn().unwrap().wait().ok();
                if Path::new(save_path.as_str()).exists() {
                    println!("Saved to: {}", save_path);
                } else {
                    println!("Unknown error while saving file");
                }
            }
        }
        None => {}
    }
}

fn screen_record(file_path: &str) {
    if let Some(users_dir) = UserDirs::new() {
        println!("Recording started: q + <enter> to stop");
        let mut child = Command::new("adb")
            .args(&["shell", "screenrecord", "/data/local/tmp/video.mp4"]).spawn().unwrap();
        loop {
            let mut buff = String::new();
            std::io::stdin().read_line(&mut buff).ok();
            if buff.trim() == "q" {
                break;
            }
        }
        child.kill().ok();
        std::thread::sleep(Duration::from_millis(500));

        let mut final_dst: String;

        if file_path == NULL {
            final_dst = users_dir.video_dir().unwrap().to_str().unwrap().to_string();
            final_dst.push_str("\\");

            print!("Enter destination: {}", final_dst);
            std::io::stdout().flush().ok();
            let mut file_name = String::new();
            std::io::stdin().read_line(&mut file_name).ok();
            file_name = file_name.trim().to_owned();
            if file_name.is_empty() {
                file_name = "video".to_owned();
            }
            if !file_name.ends_with(".mp4") {
                file_name.push_str(".mp4");
            }

            final_dst.push_str(file_name.as_str());
        } else {
            final_dst = file_path.to_owned();
        }

        Command::new("adb").args(&["pull", "/data/local/tmp/video.mp4", final_dst.as_str()]).spawn().unwrap().wait().ok();
        Command::new("adb").args(&["shell", "rm", "/data/local/tmp/video.mp4"]).spawn().unwrap().wait().ok();
    } else {
        println!("- Error: Could not detect user's dir")
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
    println!("      rec, [file-path]     Start recording device screen. Optionally, you can specify a path to save.");
    println!("      dlk, deeplink [url]  Fires a deeplink with the \"url\".");
    println!();
    println!("Examples:");
    println!("      aow");
    println!("      aow -d");
}