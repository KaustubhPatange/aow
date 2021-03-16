#[cfg(target_os = "windows")]
extern crate wfd;
#[cfg(target_os = "windows")]
use self::wfd::{DialogError, DialogParams};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use rfd::FileDialog;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn launch_windows_save_dialog() -> Result<String, ()> {
    let file = FileDialog::new().add_filter("png", &["png"]).save_file();
    match file {
        Some(buff) => {
            Ok(buff.as_path().to_str().unwrap().to_owned())
        }
        None => { Err(()) }
    }
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