use std::{ffi::CString, os::raw::c_char};

use log::error;

// only use for passing to other language
pub fn string_to_cstring_ptr(string: &str) -> Result<*const c_char, ()> {
    let name_cstr = match CString::new(string) {
        Ok(name_cstr) => name_cstr,
        Err(err) => {
            error!("failed to turn {} into cstring err {:?}", string, err);
            return Err(());
        },
    };
    let name_ptr = name_cstr.as_ptr();
    std::mem::forget(name_cstr);
    Ok(name_ptr)
}
