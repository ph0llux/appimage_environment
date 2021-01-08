/************************************************************************
* pk:56a5e5e6e74140b0d28d3b4ee3dc87b9b4d6db17864f89855d2ee3396ee27177
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