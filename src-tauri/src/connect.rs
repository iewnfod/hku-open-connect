use std::{io::{BufRead, BufReader, Write}, process::{Command, Stdio}, thread, time::Duration};

use tauri::{AppHandle, Emitter};

use crate::TOTP;

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

	pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let host = self.host.clone();
		let username = self.username.clone();
		let password = self.password.clone();
		let app_handle = self.app_handle.clone();

		thread::spawn(move || {
			let mut command = Command::new("openconnect");
			command.arg(&host);
			command.args(vec![
				"-u",
				&username
			]);
			command.arg("--passwd-on-stdin");

			command.stdin(Stdio::piped());
			command.stdout(Stdio::piped());
			command.stderr(Stdio::piped());

			println!("{:?}", &command);

			let mut child = command.spawn().unwrap();

			let mut stdin = child.stdin.take().expect("Failed to open stdin");
			writeln!(stdin, "{}", &password).unwrap();
			stdin.flush().unwrap();

			let stderr = child.stderr.take().expect("Failed to open stderr");
			let err_reader = BufReader::new(stderr);
			let app_handle = app_handle.clone();
			let err_handler = thread::spawn(move || {
				for line in err_reader.lines() {
					if let Ok(line) = line {
						println!("{}", &line);

						if let Some(ref app) = app_handle {
							if line.contains("Enter Your Microsoft verification code") {
								app.emit("totp", "").unwrap();
								loop {
									thread::sleep(Duration::from_secs(1));
									unsafe {
										#[allow(static_mut_refs)]
										if let Ok(mut totp) = TOTP.try_lock() {
											if totp.is_some() {
												let t = totp.clone().unwrap();
												writeln!(stdin, "{}", t.trim()).unwrap();
												*totp = None;
												break;
											}
										}
									}
								}
							} else if line.contains("Login failed") {
								app.emit("login-failed", "").unwrap();
							} else if line.contains("Failed to") {
								app.emit("disconnect", "").unwrap();
							}
						}
					}
				}
			});

			let status = child.wait().unwrap();
			println!("OpenConnect exit with code {:?}", status);

			err_handler.join().unwrap();

		});

		Ok(())
	}
}

impl Default for VpnClient {
	fn default() -> Self {
		Self::new()
	}
}
