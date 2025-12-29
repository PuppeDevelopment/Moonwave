use core::ffi::{c_char, c_int, c_void};
use std::ffi::CString;
use crate::opts;
use crate::dobby::DobbyHook;
use log::{error, info};
use crate::curl_hook::{setopt_hook};

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
}

const RTLD_NOW: c_int = 2;

pub unsafe fn init_ue_hook() {
    info!("Loading Unreal lib...");
    let lib_name = CString::new("libUE4.so").unwrap(); // change this to libUnreal.so if ur doing s19+
    let handle = unsafe { dlopen(lib_name.as_ptr(), RTLD_NOW) };
    if handle.is_null() {
        error!("Failed to load Unreal lib");
        return;
    }

    let mut curl_easy_setopt: *mut c_void = std::ptr::null_mut();
    if opts::USE_CURL_SYMBOL {
        let sym_name = CString::new("curl_easy_setopt").unwrap();
        curl_easy_setopt = unsafe { dlsym(handle, sym_name.as_ptr()) };
        if curl_easy_setopt.is_null() {
            error!("Failed to find curl_easy_setopt symbol");
            unsafe { dlclose(handle) };
            return;
        }

        info!("Found curl_easy_setopt at {:p}", curl_easy_setopt);
    } // TODO: add address handling for 18.40+

    let result = unsafe { DobbyHook(
        curl_easy_setopt,
        setopt_hook as *mut c_void,
        &raw mut crate::curl_hook::OG_SETOPT as *mut _ as *mut *mut c_void,
    ) };

    if result == 0 {
        info!("Successfully hooked curl_easy_setopt");
    } else {
        error!("Failed to hook curl_easy_setopt");
    }
}

// TODO: EOS Hook
/*
pub unsafe fn init_eos_hook() {
  
}
*/