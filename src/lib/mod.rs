/************************************************************************
* pk:a84fa6596f3094b3c63788f67bc7d714d4c0cf6ba36656f1c6d303b2a421c5a8
************************************************************************/
/*!
This crate contains some structs and functions to handle the tools, which are included in the AppImage.
*/

// extern crates
extern crate phollaits;
extern crate walkdir;
extern crate is_executable;

// - modules
mod environment;
//mod internal_command;

pub use environment::*;
//pub use internal_command::*;

// - Environment Vars
const ENV_VAR_APPIMAGE: &str = "APPIMAGE";
const ENV_VAR_APPDIR: &str = "APPDIR";
const ENV_VAR_OWD: &str = "OWD";
const ENV_VAR_ARG0: &str = "ARGV0";

/// contains all folders inside the appimage, which can contain executable binaries.
pub const PATH: [&'static str; 4] = [ "/usr/local/bin", "/usr/bin", "/bin", "/usr/games" ];

// - SPECIAL FOLDERS
const SPECIAL_PATH_DATA: &str = "/data";
const SPECIAL_PATH_WEB: &str = "/web";

// - Errors
const NOT_FOUND: &str = "not found";

// - Separators
//const SEPARATOR_DOUBLE_QUOTE: &str = "\"";