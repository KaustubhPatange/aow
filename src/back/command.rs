use crate::back::device::Device;
use crate::back::connect::disconnect;

pub fn parse_command(c: &Vec<String>) {
    if c.contains(&"-h".to_owned()) || c.contains(&"--help".to_owned()) {
        print_all_commands()
    } else if c[1].eq("-s") || c[1].eq("--show") {
        let _ = show_connected_device();
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

fn print_all_commands() {
    println!("ADB Over Wifi (aow) v0.1.0 - A command line tool to connect devices over wifi (requires ADB).");
    println!("Copyright 2020 Kaustubh Patange - https://github.com/KaustubhPatange/aow");
    println!();
    println!("Usage: aow [options]");
    println!();
    println!("Options:");
    println!("      [null]              Connects a device over wifi (see demo on Github)");
    println!("      -s, --show          Shows the connected device over wifi (if any).");
    println!("      -d, --disconnect    Disconnect the connected device (if any).");
    println!("      -h, --help          Prints this help message.");
    println!();
    println!("Examples:");
    println!("      aow");
    println!("      aow -d");
}