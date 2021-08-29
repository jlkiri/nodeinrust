use scrypt::execute_js_file;
use std::{ffi::CString};

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: nodeinrust FILE");
        std::process::exit(1);
    });
    let filepath = CString::new(arg).expect("Invaid CString");
    let ptr = filepath.into_raw();

    unsafe {
        execute_js_file(ptr);
        let _ = CString::from_raw(ptr);
    }
}
