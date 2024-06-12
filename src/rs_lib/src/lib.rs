extern crate libc;
use libc::malloc;
use std::ffi::CString;
use std::ptr;

#[no_mangle]
pub extern "C" fn rs_lib_func() -> *mut libc::c_char {
    unsafe {
        let rust_string = "Hello from rust!";
        let c_string = CString::new(rust_string).expect("CString::new failed");
        let len = c_string.as_bytes_with_nul().len();
        let buffer = malloc(len) as *mut u8;

        // Check if malloc failed
        if buffer.is_null() {
            return ptr::null_mut();
        }

        // Copy the CString into the malloc'd buffer
        ptr::copy_nonoverlapping(c_string.as_ptr(), buffer as *mut libc::c_char, len);

        buffer as *mut libc::c_char
    }
}
