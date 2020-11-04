use {
    crate::back::device::{Device, Status},
    std::process::{Command, Stdio},
    regex::Regex,
    wait_timeout::ChildExt,
    std::time::Duration,
    std::io::Read,
};

pub fn connect_device(d: &Device) {
    match d.status {
        Status::ONLINE => {
            if !is_device_connected_to_wifi(d) {
                println!("- Error: Device {} is not connected to Wifi", d.device_id);
                return
            }

            let re = Regex::new(r"inet\saddr:(192\.168\.\d\.\d{2,})").unwrap();

            let out = Command::new("adb")
                .args(&["-s", &d.device_id[..], "shell", "ifconfig"])
                .output().unwrap();

            let ip = String::from_utf8_lossy(&out.stdout);
            let ip = &re.captures(&ip).unwrap()[1];

            println!("- address: {}", ip);
            println!("Making a connection...");

            let mut child = Command::new("adb").arg("connect").arg(format!("{}:5555", ip))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let _status_code = match child.wait_timeout(Duration::from_secs(5)).unwrap() {
                Some(status) => {
                    let mut buffer = String::new();
                    child.stdout.unwrap().read_to_string(&mut buffer).unwrap();

                    println!("- {}", buffer);

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
        Status::OFFLINE => {
            println!("- Error: Device {} is offline", d.device_id);
            println!();
            println!("Hint: Try disconnecting & re-connecting device.")
        }
        Status::UNAUTHORIZED => {
            println!("- Error: Device {} is unauthorized", d.device_id);
            println!();
            println!("Hint: Accept the prompt in your device & re run the command.")
        }
    }
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