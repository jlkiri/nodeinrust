use std::os::raw::c_char;

extern "C" {
    pub fn execute_node(file: *mut c_char) -> i32;
}
