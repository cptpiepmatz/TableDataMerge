#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=icon/icon.ico");
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon/icon.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
