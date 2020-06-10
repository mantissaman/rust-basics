use std::os::raw::{c_char, c_uint};
use std::ffi::CString;

extern "C" {
    fn mystrlen(str: *const c_char) -> c_uint;
}


fn safe_mystrlen(str: &str) -> Option<u32> { 
    let c_string = match CString::new(str) { 
        Ok(c) => c, 
        Err(_) => return None 
    };

    unsafe { 
        Some(mystrlen(c_string.as_ptr())) 
    } 
} 

fn main() {
    let c_string = CString::new("C from Rust").expect("failed");
    let count = unsafe{
        mystrlen(c_string.as_ptr())
    };
    println!("c_string length is {}", count);

    let safe_count = safe_mystrlen("C from Rust").unwrap();
    println!("safe c_string length is {}", safe_count);
}
