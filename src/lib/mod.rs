/************************************************************************
* pk:f03b8be823ec01b0d6112c0001d4bcab44d5aa24280759788019e24ea8a3ffb7
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
mod internal_command;

pub use environment::*;
pub use internal_command::*;

// - Environment Vars
const ENV_VAR_APPIMAGE: &str = "APPIMAGE";
const ENV_VAR_APPDIR: &str = "APPDIR";
const ENV_VAR_OWD: &str = "OWD";
const ENV_VAR_ARG0: &str = "ARGV0";

/// contains all folders inside the appimage, which can contain executable binaries.
/// Currently folllowing paths inside the appimage are interpreted as binary-paths:
/// */usr/local/bin*, */usr/bin*, */bin*, */usr/games*.
pub const PATH: [&'static str; 4] = [ "/usr/local/bin", "/usr/bin", "/bin", "/usr/games" ];

// - SPECIAL FOLDERS
const SPECIAL_PATH_DATA: &str = "/data";
const SPECIAL_PATH_WEB: &str = "/web";

// - Errors
const NOT_FOUND: &str = "not found";
const ERROR_CMDLINE_QUOTES: &str = "There are a wrong number of quotes.";
const ERROR_EMPTY_STRING: &str = "Empty string.";
const ERROR_NOT_SET: &str = "not set.";

// - SEPARATORS
const SEPARATOR_DOUBLE_QUOTE: &str = "\"";
const SEPARATOR_PIPE: &str = "|";