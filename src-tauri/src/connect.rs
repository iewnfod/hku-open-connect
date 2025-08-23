#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{io::{BufReader, Read, Write}, path::PathBuf, process::{Command, Stdio}, sync::Mutex, thread::{self, JoinHandle}, time::Duration};

use tauri::{AppHandle, Emitter};

use crate::{utils::get_openconnect_path, OPENCONNECT_CHILD_ID, TOTP};


static mut STDIN_QUEUE: Mutex<Vec<String>> = Mutex::new(vec![]);

pub struct VpnClient {
	host: String,
	username: String,
	password: String,
	app_handle: Option<AppHandle>
}

impl VpnClient {
	fn new() -> Self {
		Self {
			username: String::new(),
			password: String::new(),
			host: String::from("vpn2fa.hku.hk"),
			app_handle: None
		}
	}

	pub fn with_username<T: ToString>(mut self, username: T) -> Self {
		self.username = username.to_string();
		self
	}

	pub fn with_password<T: ToString>(mut self, password: T) -> Self {
		self.password = password.to_string();
		self
	}

	pub fn _with_host<T: ToString>(mut self, host: T) -> Self {
		self.host = host.to_string();
		self
	}

	pub fn with_app_handle(mut self, app_handle: AppHandle) -> Self {
		self.app_handle = Some(app_handle);
		self
	}

	pub fn connect(&mut self) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
		let host = self.host.clone();
		let username = self.username.clone();
		let password = self.password.clone();
		let app_handle = self.app_handle.clone();
		let success_text = format!("Legacy IP route configuration done.");
		println!("OpenConnect will success with: {}", &success_text);

		let open_connect_handler = thread::spawn(move || {
			let mut openconnect_path = PathBuf::from("openconnect");
			if let Some(ref app) = app_handle {
				openconnect_path = get_openconnect_path(&app);
			}

			let mut command = Command::new(openconnect_path);
			command.arg(&host);
			command.args(vec![
				"-u",
				&username
			]);
			command.arg("--passwd-on-stdin");

			#[cfg(windows)]
			command.creation_flags(winapi::um::winbase::CREATE_NEW_PROCESS_GROUP);

			command.stdin(Stdio::piped());
			command.stdout(Stdio::piped());
			command.stderr(Stdio::piped());

			println!("{:?}", &command);

			let mut child = command.spawn().unwrap();

			let child_id = child.id();
			unsafe {
				#[allow(static_mut_refs)]
				if let Ok(mut c) = OPENCONNECT_CHILD_ID.lock() {
					*c = Some(child_id);
				}
			}

			let mut stdin = child.stdin.take().expect("Failed to open stdin");
			writeln!(stdin, "{}", &password).unwrap();
			stdin.flush().unwrap();

			let stdout = child.stdout.take().expect("Failed to open stdout");
			let stderr = child.stderr.take().expect("Failed to open stderr");

			let mut err_reader = BufReader::new(stderr);
			let mut out_reader = BufReader::new(stdout);

			let app_handle1 = app_handle.clone();
			let out_handler = thread::spawn(move || {
				let mut buffer = String::new();
				let mut char_bf = [0; 1];

				loop {
					if let Some(ref app) = app_handle1 {
						if let Ok(c) = out_reader.read(&mut char_bf) {
							if c == 0 {
								app.emit("disconnect", "").unwrap();
								println!("Disconnect");
								break;
							}

							let c = char_bf[0] as char;
							buffer.push(c);
							print!("{}", c);
						}

						check_buffer(&mut buffer, app);
					}
				}
			});

			let app_handle2 = app_handle.clone();
			let err_handler = thread::spawn(move || {
				let mut buffer = String::new();
				let mut char_bf = [0; 1];

				loop {
					if let Some(ref app) = app_handle2 {
						if let Ok(c) = err_reader.read(&mut char_bf) {
							if c == 0 {
								app.emit("disconnect", "").unwrap();
								println!("Disconnect");
								break;
							}

							let c = char_bf[0] as char;
							buffer.push(c);
							print!("{}", c);
						}

						check_buffer(&mut buffer, app);
					}
				}
			});

			let stdin_handler = thread::spawn(move || {
				loop {
					unsafe {
						#[allow(static_mut_refs)]
						if let Ok(mut in_queue) = STDIN_QUEUE.try_lock() {
							while let Some(v) = in_queue.pop() {
								writeln!(stdin, "{}", &v).unwrap();
							}
						}
					}
					thread::sleep(Duration::from_millis(160));
				}
			});

			let status = child.wait().unwrap();
			println!("OpenConnect exit with status {:?}", status);

			out_handler.join().unwrap();
			err_handler.join().unwrap();
			stdin_handler.join().unwrap();
		});

		Ok(open_connect_handler)
	}
}

impl Default for VpnClient {
	fn default() -> Self {
		Self::new()
	}
}

fn check_buffer(buffer: &mut String, app: &AppHandle) {
	if buffer.contains("Enter Your Microsoft verification code") {
		app.emit("totp", "").unwrap();
		buffer.clear();
		loop {
			thread::sleep(Duration::from_secs(1));
			unsafe {
				#[allow(static_mut_refs)]
				if let Ok(mut totp) = TOTP.try_lock() {
					if totp.is_some() {
						let t = totp.clone().unwrap();
						add_stdin(t.trim());
						*totp = None;
						break;
					}
				}
			}
		}
	} else if buffer.contains("Login failed") {
		app.emit("login-failed", "").unwrap();
		buffer.clear();
	} else if buffer.contains("Failed to") {
		app.emit("disconnect", "").unwrap();
		buffer.clear();
	} else if buffer.contains("Legacy IP route configuration done") {
		app.emit("connected", "").unwrap();
		buffer.clear();
	}
}

fn add_stdin<T: ToString>(value: T) {
	unsafe {
		#[allow(static_mut_refs)]
		if let Ok(mut in_queue) = STDIN_QUEUE.lock() {
			in_queue.push(value.to_string());
		}
	}
}
