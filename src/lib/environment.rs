/************************************************************************
* pk:c4e0aa9cc15239b26be0e68444b714d1e3b62d6773b13672ef9d33dd4ff6529a
************************************************************************/
/*!
Environment
*/

// - STD
use std::io;
use std::env;
use std::path::{PathBuf, Path};

// - internal
use crate as appimage_environment;

// - external
use walkdir::WalkDir;
use phollaits::{ToIOResult};
use is_executable::IsExecutable;

/// The struct Environment contains some environment-informations about the appimage. For example, you can get
/// the values of the appimage environment variables or the HashMap of into the appimage included programs.
pub struct Environment;

impl Environment {
	/// If the binary lies into an appimage, this method returns the appimage environment variable "APPDIR" (absolute
	/// path to AppImage file, with symlinks resolved) See [AppImage documentation](https://docs.appimage.org/packaging-guide/environment-variables.html)
	/// for more details.
	pub fn appdir() -> io::Result<String> {
		env::var(appimage_environment::ENV_VAR_APPDIR).to_io_result()
	}

	/// If the binary lies into an appimage, this method returns the appimage environment variable "APPIMAGE".
	/// See [AppImage documentation](https://docs.appimage.org/packaging-guide/environment-variables.html)
	/// for more details.
	pub fn appimage() -> io::Result<String> {
		env::var(appimage_environment::ENV_VAR_APPIMAGE).to_io_result()
	}

	/// If the binary lies into an appimage, this method returns the appimage environment variable "OWD".
	/// See [AppImage documentation](https://docs.appimage.org/packaging-guide/environment-variables.html)
	/// for more details.
	pub fn owd() -> io::Result<String> {
		env::var(appimage_environment::ENV_VAR_OWD).to_io_result() //same output as PWD
	}

	/// If the binary lies into an appimage, this method returns the appimage environment variable "ARGV0".
	/// See [AppImage documentation](https://docs.appimage.org/packaging-guide/environment-variables.html)
	/// for more details.
	pub fn argv0() -> io::Result<String> {
		env::var(appimage_environment::ENV_VAR_ARG0).to_io_result()
	}

	/// If the binary lies into an appimage, this method will attempt to return the full path of the given toolname.
	/// # Example
	/// ```rustc
	/// extern crate appimage_environment;
	/// use appimage_environment::Environment;
	/// 
	///	fn main() {
	///		println!("My examplebinary: {:?}", Environment::get_path_of("lspci"));
	/// }
	/// ```
	pub fn get_path_of<S: Into<String>>(toolname: S) -> io::Result<PathBuf> {
		let toolname = toolname.into();
		let appdir = Environment::appdir()?;
		for path in appimage_environment::PATH.iter() {
			let path = format!("{}{}", appdir, path);
			for entry in WalkDir::new(path).min_depth(1).max_depth(1)
			{
				let entry = match entry {
					Ok(entry) => entry,
					Err(_) => continue,
				};
				let path = entry.into_path();
				if path.is_executable() {
					if path.to_str().to_io_result()?.ends_with(&toolname) {
						return Ok(path);
					}
				}
			}
		}
		return Err(io::Error::new(io::ErrorKind::Other, appimage_environment::NOT_FOUND));
	}

	/// This function returns all paths of binaries which are in the appropriate binary paths of the appimage.
	/// The appropriate paths can be found in [PATH](constant.PATH.html).
	pub fn get_all_bins() -> io::Result<Vec<PathBuf>> {
		let mut binaries = Vec::new();
		let appdir = Environment::appdir()?;
		for path in appimage_environment::PATH.iter() {
			let path = format!("{}{}", appdir, path);
			for entry in WalkDir::new(path).min_depth(1).max_depth(1)
			{
				let entry = match entry {
					Ok(entry) => entry,
					Err(_) => continue,
				};
				let path = entry.into_path();
				if path.is_executable() {
					binaries.push(path);
				}
			}
		}
		Ok(binaries)
	}

	/// This function returns the path of "/data" of the appimage.
	pub fn get_data_path() -> io::Result<PathBuf> {
		let appdir = Environment::appdir()?;
		let path = format!("{}{}", appdir, appimage_environment::SPECIAL_PATH_DATA);
		Ok(Path::new(&path).to_path_buf())
	}

	/// This function returns the path of "/web" of the appimage.
	pub fn get_web_path() -> io::Result<PathBuf> {
		let appdir = Environment::appdir()?;
		let path = format!("{}{}", appdir, appimage_environment::SPECIAL_PATH_WEB);
		Ok(Path::new(&path).to_path_buf())
	}
}