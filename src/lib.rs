use std::os::raw::c_char;

extern "C" {
    pub fn execute_js_file(path: *mut c_char) -> i32;
}
