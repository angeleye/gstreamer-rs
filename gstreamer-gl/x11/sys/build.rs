// This file was generated by gir (https://github.com/gtk-rs/gir @ 6ec2baf)
// from gir-files (https://github.com/gtk-rs/gir-files @ 9424eabd)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}