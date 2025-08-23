use std::{env::temp_dir, fs, io, path::{Path, PathBuf}};

use tauri::{AppHandle, Manager};

#[cfg(target_os="windows")]
const PLATFORM: &str = "windows";
#[cfg(target_os="macos")]
const PLATFORM: &str = "macos";
#[cfg(target_os="linux")]
const PLATFORM: &str = "linux";

pub fn get_openconnect_path(app: &AppHandle) -> PathBuf {
	let temp_bin_dir = get_temp_bin_dir();
	#[cfg(target_os="windows")]
	let bin = temp_bin_dir.join("openconnect.exe");
	#[cfg(not(target_os="windows"))]
	let bin = temp_bin_dir.join("openconnect");

	if !bin.exists() {
		extract_embedded_binaries(app);
	}

	bin
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn extract_embedded_binaries(app: &AppHandle) {
	let app_dir = app.path();
	let resource_dir = app_dir.resource_dir().expect("Failed to get resource dir");
	let temp_bin_dir = get_temp_bin_dir();

	fs::create_dir_all(&temp_bin_dir).unwrap();

	let source_bin_dir = resource_dir.join("bin").join(PLATFORM);

	if source_bin_dir.exists() {
		copy_dir_all(&source_bin_dir, &temp_bin_dir).unwrap();
	}
}

fn get_temp_bin_dir() -> PathBuf {
	temp_dir().join("hku-vpn-openconnect")
}
