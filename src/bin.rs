use scrypt::execute_node;
use std::{ffi::CString, fs, path::PathBuf};

fn main() {
    //let path = PathBuf::from("./node/out/Release");
    //let libpath = fs::canonicalize(path).expect("efef");
    //println!(
    //    "cargo:rustc-link-search={}",
    //    libpath.to_str().expect("Bad path")
    //);

    let args: Vec<String> = std::env::args().collect();
    let filepath = CString::new(args[1].clone()).expect("CString failed");
    let ptr = filepath.into_raw();
    unsafe {
        execute_node(ptr);
        let _ = CString::from_raw(ptr);
    }
}
