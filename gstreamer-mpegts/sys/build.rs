// This file was generated by gir (https://github.com/gtk-rs/gir @ 0ae3977)
// from gir-files (https://github.com/gtk-rs/gir-files @ 27dc5b40)
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
