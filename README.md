<img src="https://img.shields.io/crates/d/appimage_environment?style=for-the-badge"/> <img src="https://img.shields.io/crates/v/appimage_environment?color=blue&logo=Rust&style=for-the-badge" href="https://crates.io/crates/appimage_environment" /> <img src="https://img.shields.io/crates/v/appimage_environment?color=blue&label=docs&logo=Rust&style=for-the-badge" href="https://docs.rs/appimage_environment"/> <img src="https://img.shields.io/crates/l/appimage_environment?style=for-the-badge"/> 


# appimage_environment
Interact in simple ways within an AppImage as a Rust program.

[Documentation](https://docs.rs/appimage_environment)

## Example usage

You could start by building a simple appimage-directory, with following structure:
```bash
mkdir -p appimage_dir/{bin,usr/local/bin,usr/bin,usr/games,data,web}
```
... and you have to add some needed stuff for appimages (yes I know...this is an annoying fact for me too):
```bash
wget "https://raw.githubusercontent.com/ph0llux/appimage_environment/main/appimage_example_stuff/init.desktop" -O appimage_dir/init.desktop
wget "https://raw.githubusercontent.com/ph0llux/appimage_environment/main/appimage_example_stuff/icon.png" -O appimage_dir/icon.png
```

... now copy some example stuff:
```bash
git clone https://github.com/ph0llux/get_ipv4_addr
cd get_ipv4_addr
make
cp get_ip_addr ../appimage_dir/usr/bin/
chmod +x ../appimage_dir/usr/src/get_ip_addr
cd ..
```

create a cargo project in normal ways and add some example code to main.rs (don't forget to include appimage_environment in your Cargo.toml!):

```rust
extern crate appimage_environment;

use appimage_environment::Environment;
use appimage_environment::InternalCommand;

fn main() {
    println!("{:?}", Environment::get_path_of("get_ip_addr"));
    println!("{:?}", InternalCommand::get_stdout_from_call("get_ip_addr"));
}
```

You can now compile the target and add them to the appimage:
```bash
cargo build --release
cp target/release/project_named_target ../appimage_dir/AppRun #your executable must be named "AppRun"
```

Now you can build your appimage and execute (example is for x86_64)!
```bash
wget "https://github.com/AppImage/AppImageKit/releases/download/12/appimagetool-x86_64.AppImage"
chmod +x appimagetool-x86_64.AppImage
./appimagetool-x86_64.AppImage -n appimage_dir
```