use {
    crate::back::device::{Device, Status},
    regex::Regex,
    std::io::Read,
    std::process::{Command, Stdio},
    std::time::Duration,
    wait_timeout::ChildExt,
};

pub fn connect_device(d: &Device) {
    match d.status {
        Status::ONLINE => {
            if !is_device_connected_to_wifi(d) {
                println!("- Error: Device {} is not connected to Wifi", d.device_id);
                return;
            }

            let re = Regex::new(r"inet\s?([\d]{3}\.[\d]{1,3}\.[\d]{1,3}\.[\d]{1,3})").unwrap();

            let out = Command::new("adb")
                .args(&["-s", &d.device_id[..], "shell", "ip", "addr", "show", "wlan0"])
                .output()
                .unwrap();

            let ip = String::from_utf8_lossy(&out.stdout);
            let captures = re.captures(&ip);
            match captures {
                Some(value) => {
                    let ip = &value[1];

                    println!("- address: {}", ip);
                    println!("Making a connection...");

                    let mut child = Command::new("adb")
                        .arg("connect")
                        .arg(format!("{}:5555", ip))
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let _status_code = match child.wait_timeout(Duration::from_secs(5)).unwrap() {
                        Some(status) => {
                            let mut buffer = String::new();
                            child.stdout.unwrap().read_to_string(&mut buffer).unwrap();

                            println!("- {}", buffer);
                            if !buffer.starts_with("cannot connect") {
                                println!("Safe to remove the USB cable along with device.");
                            }else {
                                println!("Applying fix: killing server");
                                Command::new("adb").arg("kill-server").spawn().unwrap().wait();
                                Command::new("adb").arg("tcpip 5555").spawn().unwrap().wait();
                                println!();
                                println!("Hint: Try running the command 'aow' again to see if error is fixed.")
                            }

                            status.code()
                        }
                        None => {
                            println!("- Error: Connection timeout!");
                            println!();
                            println!("Hint:\n1. {}\n2. {}",
                                     "Disconnect & re-connect your wifi of client device.",
                                     "Check if mobile data is ON along with Wifi (this may sometimes causes a reason for connection failure).");

                            child.kill().unwrap();
                            child.wait().unwrap().code()
                        }
                    };
                }
                None => {
                    println!("- Error: Couldn't find IP address");
                    println!();
                    println!("Hint: If your device is connected to WIFI & still you are getting this error, contact me quickly on Github.");
                    return;
                }
            }
        }
        Status::OFFLINE => {
            println!("- Error: Device {} is offline", d.device_id);
            println!();
            println!("Hint: Try disconnecting & re-connecting device or use aow -d to disconnect from all devices.")
        }
        Status::UNAUTHORIZED => {
            println!("- Error: Device {} is unauthorized", d.device_id);
            println!();
            println!("Hint: Accept the prompt in your device & re run the command.")
        }
    }
}

pub fn disconnect() {
    Command::new("adb").arg("disconnect").spawn().unwrap().wait();
}

fn is_device_connected_to_wifi(d: &Device) -> bool {
    let output = Command::new("adb")
        .args(&["-s", &d.device_id[..], "shell", "dumpsys", "connectivity"])
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout);

    let re = Regex::new(r"NetworkAgentInfo(.*?)WIFI\[],(.*?)CONNECTED/CONNECTED").unwrap();

    return re.is_match(&output);
}
