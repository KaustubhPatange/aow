#[cfg(target_os = "windows")]
extern crate wfd;
#[cfg(target_os = "windows")]
use self::wfd::{DialogError, DialogParams};

#[cfg(any(target_os = "linux", target_os = "mac"))]
pub fn launch_windows_save_dialog() -> Result<String, ()> {
    println!("- Error: Native dialogs are not supported on {}", std::env::consts::OS);
    Err(())
}

#[cfg(target_os = "windows")]
pub fn launch_windows_save_dialog() -> Result<String, ()> {
    let params = DialogParams {
        file_name: "file.png",
        file_types: vec![("png", "*.png")],
        title: "Choose a path to save",
        ..Default::default()
    };
    let result = wfd::save_dialog(params);
    match result {
        Ok(file) => {
            Ok(String::from(file.selected_file_path.to_str().unwrap()))
        }
        Err(e) => {
            match e {
                DialogError::HResultFailed { hresult, error_method} => {
                    println!("- Error: HResult Failed - HRESULT: {:X}, Method: {}", hresult, error_method);
                }
                DialogError::UnsupportedFilepath => { println!("- Error: Unsupported file path"); }
                DialogError::UserCancelled => { }
            }
            Err(())
        }
    }
}