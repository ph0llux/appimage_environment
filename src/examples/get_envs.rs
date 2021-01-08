/************************************************************************
* pk:
************************************************************************/

extern crate appimage_environment;
use appimage_environment::Environment;

fn main() {
	println!("appdir: {:?}", Environment::appdir());
	println!("appimage: {:?}", Environment::appimage());
	println!("owd: {:?}", Environment::owd());
	println!("argv0: {:?}", Environment::argv0());
	println!("Path of Binary 'lspci' {:?}:", Environment::get_path_of("lspci"));
}