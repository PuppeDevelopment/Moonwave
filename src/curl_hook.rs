use std::ffi::{CStr, CString, c_char};
use std::os::raw::{c_int, c_void};
use log::info;
use curl_sys::{CURLOPT_SSL_VERIFYPEER, CURLOPT_URL};
use crate::opts;
use crate::url::Url;

pub static mut OG_SETOPT: Option<extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int> = None;
pub static mut EOS_OG_SETOPT: Option<extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int> = None;

pub extern "C" fn setopt_hook(handle: *mut c_void, option: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        // compiler..... sybau please
        let og_setopt = OG_SETOPT.expect("OG_SETOPT is not initialized!");

        if option == CURLOPT_URL as c_int && !arg.is_null() {
            let url_cstr = CStr::from_ptr(arg as *const c_char);
            if let Ok(url_str) = url_cstr.to_str() {
                let url = Url::parse_url(url_str);
                //info!("Original Host: {}\nOriginal Path and Query: {}", url.host, url.path_and_query);

                if Url::should_redirect(&url.host) {
                    let redirected = Url::create_url(opts::BACKEND_URL, &url.path_and_query);
                    //info!("Redirected URL from {} to {}", url_str, redirected);

                    let redirected_cstr = CString::new(redirected).unwrap().into_raw();
                    return og_setopt(handle, option, redirected_cstr as *mut c_void);
                }
            }
        } else if option == CURLOPT_SSL_VERIFYPEER as c_int && opts::BYPASS_SSL {
            return og_setopt(handle, option, 0usize as *mut c_void);
        }

        og_setopt(handle, option, arg)
    }
}

pub extern "C" fn eos_setopt_hook(handle: *mut c_void, option: c_int, arg: *mut c_void) -> c_int {
    unsafe {
        // compiler..... sybau please
        let og_setopt = EOS_OG_SETOPT.expect("EOS_OG_SETOPT is not initialized!");

        if option == CURLOPT_URL as c_int && !arg.is_null() {
            let url_cstr = CStr::from_ptr(arg as *const c_char);
            if let Ok(url_str) = url_cstr.to_str() {
                let url = Url::parse_url(url_str);
                //info!("Original Host: {}\nOriginal Path and Query: {}", url.host, url.path_and_query);

                if Url::should_redirect(&url.host) {
                    let redirected = Url::create_url(opts::BACKEND_URL, &url.path_and_query);
                    //info!("Redirected URL from {} to {}", url_str, redirected);

                    let redirected_cstr = CString::new(redirected).unwrap().into_raw();
                    return og_setopt(handle, option, redirected_cstr as *mut c_void);
                }
            }
        } else if option == CURLOPT_SSL_VERIFYPEER as c_int && opts::BYPASS_SSL {
            return og_setopt(handle, option, 0usize as *mut c_void);
        }

        og_setopt(handle, option, arg)
    }
}
