use std::{ffi::CString, os::raw::c_char};

extern "C" {
    fn execute_node(file: *mut c_char) -> i32;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = CString::new(args[1].clone()).expect("CString failed");
    let ptr = filepath.into_raw();
    unsafe {
        execute_node(ptr);
        let _ = CString::from_raw(ptr);
    }
}
