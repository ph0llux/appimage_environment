/************************************************************************
* pk:192e007b0347181a117ebde5b36fa80481e23d4eba761ac796c6f8de37fd1ea7
************************************************************************/
// - STD
use std::collections::VecDeque;
use std::io;
use std::process::{Command, Output, Stdio};

// - internal
use crate as appimage_environment;
use appimage_environment::Environment as Env;

// - external
use phollaits::{BoolExtensions, ToIOResult};

///The struct [InternalCommand](struct.InternalCommand.html) contains several methods to use commands of appimage internal binaries.
pub struct InternalCommand;

impl InternalCommand {
	fn vectorize_command<C: Into<String>>(command: C) -> io::Result<VecDeque<VecDeque<String>>> {
		let command = command.into();
		if command.contains(appimage_environment::SEPARATOR_DOUBLE_QUOTE)
			&& command.matches(appimage_environment::SEPARATOR_DOUBLE_QUOTE).count() % 2 == 1
		{
			return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_CMDLINE_QUOTES));
		}

		let command_vectorized = {
			let split = command.split(appimage_environment::SEPARATOR_PIPE);
			split.collect::<VecDeque<&str>>()
		};
		let command_vectorized = {
			let mut commands = VecDeque::new();
			for command in command_vectorized {
				let mut split = command.split(appimage_environment::SEPARATOR_DOUBLE_QUOTE);
				let mut argument = false;
				let mut element = VecDeque::new();
				loop {
					match split.next() {
						Some(x) => {
							if !argument {
								element.push_back(x.to_string())
							} else {
								element.push_back(appimage_environment::SEPARATOR_DOUBLE_QUOTE.to_string());
								element.push_back(x.to_string());
								element.push_back(appimage_environment::SEPARATOR_DOUBLE_QUOTE.to_string());
							}
						}
						None => break,
					}
					argument.reverse();
				}
				commands.push_back(element);
			}
			commands
		};
		let command_vectorized = {
			let mut commands = VecDeque::new();
			for command in command_vectorized {
				let mut cmd = VecDeque::new();
				for element in command {
					if !element.contains(appimage_environment::SEPARATOR_DOUBLE_QUOTE) {
						let mut split = element.split_whitespace();
						loop {
							match split.next() {
								Some(x) => {
									cmd.push_back(x.to_string());
								}
								None => break,
							}
						}
					} else {
						cmd.push_back(element.to_string());
					}
				}
				commands.push_back(cmd);
			}
			commands
		};
		Ok(command_vectorized)
	}

	/// returns the full [Output](https://doc.rust-lang.org/std/process/struct.Output.html)-instance of the called
	/// command-chain. You can use the method like:
	/// ```rustc
	/// fn main() {
	/// 	let output = app_image_tools::InternalCommand::call("lspci").unwrap();
	/// 	let stdout = String::from_utf8_lossy(&output.stdout).to_string();
	/// 	let stderr = String::from_utf8_lossy(&output.stderr).to_string();
	/// 	let status_code = output.status.success(); //returns a bool
	/// 	println!("stdout: {}", stdout);
	/// 	println!("stderr: {}", stderr);
	/// 	println!("Status-Code: {}", status_code);
	/// }
	/// ```
	/// the method will call a command, if the command is available in the AppImage environment. If not, the
	/// method returns a [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html).
	pub fn call<C: Into<String>>(command: C) -> io::Result<Output> {
		let mut command_vectorized = InternalCommand::vectorize_command(command)?;
		if command_vectorized.len() == 1 {
			let mut cmdline = match command_vectorized.pop_front() {
				Some(x) => x,
				None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_EMPTY_STRING)),
			};
			let cmd = {
				let toolname = match cmdline.pop_front() {
					Some(x) => x,
					None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_NOT_SET)),
				};
				Env::get_path_of(&toolname)?
			};
			let mut args = VecDeque::new();
			args.append(&mut cmdline);
			Command::new(cmd).args(args.into_iter()).output()
		} else if command_vectorized.len() > 1 {
			let mut last_cmdline = command_vectorized.pop_back().to_io_result()?;
			let mut cmdline = match command_vectorized.pop_front() {
				Some(x) => x,
				None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_EMPTY_STRING)),
			};
			let cmd = {
				let toolname = match cmdline.pop_front() {
					Some(x) => x,
					None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_NOT_SET)),
				};
				Env::get_path_of(&toolname)?
			};
			let mut args = VecDeque::new();
			args.append(&mut cmdline);
			let mut child = Command::new(cmd).args(args.into_iter()).spawn()?;
			loop {
				let mut cmdline = match command_vectorized.pop_front() {
					Some(x) => x,
					None => break,
				};
				let cmd = {
					let toolname = match cmdline.pop_front() {
						Some(x) => x,
						None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_NOT_SET)),
					};
					Env::get_path_of(&toolname)?
				};
				let mut args = VecDeque::new();
				args.append(&mut cmdline);
				child = match child.stdout {
					Some(x) => Command::new(cmd).args(&args).stdin(x).stdout(Stdio::piped()).spawn()?,
					None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_EMPTY_STRING)),
				}
			}
			let cmd = {
				let toolname = match last_cmdline.pop_front() {
					Some(x) => x,
					None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_NOT_SET)),
				};
				Env::get_path_of(&toolname)?
			};
			let mut args = VecDeque::new();
			args.append(&mut last_cmdline);
			match child.stdout {
				Some(x) => return Command::new(cmd).args(&args).stdin(x).output(),
				None => return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_EMPTY_STRING)),
			}
		} else {
			return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::ERROR_EMPTY_STRING));
		}
	}

	/// returns the stdout of the called command-chain as String. You can use the method like:
	/// ```rustc
	/// fn main() {
	/// 	let stdout = app_image_tools::InternalCommand::get_stdout_from_call("lspci").unwrap();
	/// 	println!("{}", stdout);
	/// ```
	/// }
	/// the method will call a command, if the command is available in the AppImage environment. If not, the
	/// method returns a [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html).
	pub fn get_stdout_from_call<C: Into<String>>(command: C) -> io::Result<String> {
		let output = InternalCommand::call(command)?;
		Ok(String::from_utf8_lossy(&output.stdout).to_string())
	}

	/// returns the stderr of the called command-chain as String. You can use the method like:
	/// ```rustc
	/// fn main() {
	/// 	let stderr = app_image_tools::InternalCommand::get_stderr_from_call("lspci").unwrap();
	/// 	println!("{}", stderr);
	/// }
	/// ```
	/// the method will call a command, if the command is available in the AppImage environment. If not, the
	/// method returns a [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html).
	pub fn get_stderr_from_call<C: Into<String>>(command: C) -> io::Result<String> {
		let output = InternalCommand::call(command)?;
		Ok(String::from_utf8_lossy(&output.stderr).to_string())
	}
}