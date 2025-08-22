use std::sync::Mutex;

use tauri::AppHandle;

use crate::connect::VpnClient;

mod connect;

pub static mut TOTP: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
fn connect_vpn(app: AppHandle, username: String, password: String) {
    let mut client = VpnClient::default()
        .with_username(username)
        .with_password(password)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect_vpn, submit_totp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
