use std::sync::Mutex;

use kill_tree::blocking::kill_tree;
use tauri::AppHandle;

use crate::connect::VpnClient;

mod connect;
mod utils;

pub static mut TOTP: Mutex<Option<String>> = Mutex::new(None);
pub static mut OPENCONNECT_CHILD_ID: Mutex<Option<u32>> = Mutex::new(None);

#[tauri::command]
fn connect_vpn(app: AppHandle, username: String, password: String, host: String) {
    let mut client = VpnClient::default()
        .with_username(username)
        .with_password(password)
        .with_host(host)
        .with_app_handle(app);

    let _ = client.connect();
}

#[tauri::command]
fn submit_totp(totp: String) {
    unsafe {
        #[allow(static_mut_refs)]
        let mut t = TOTP.lock().unwrap();
        *t = Some(totp);
    }
}

#[tauri::command]
fn disconnect_vpn() {
    unsafe {
        #[allow(static_mut_refs)]
        if let Ok(mut c) = OPENCONNECT_CHILD_ID.lock() {
            if c.is_some() {
                let child_id = c.take().unwrap();
                println!("Try to kill process {}", child_id);
                match kill_tree(child_id) {
                    Ok(_) => {
                        println!("Kill process success.");
                    },
                    Err(_) => {
                        println!("Kill process failed.");
                    }
                }
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect_vpn, submit_totp, disconnect_vpn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
