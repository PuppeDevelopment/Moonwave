use objc::runtime::{Class, Object, Sel, Imp};
use objc::{msg_send, sel, sel_impl};
use crate::opts::{BACKEND_URL};
use std::ffi::CString;
use std::ptr;

// ts tuff icl frfr peak
pub unsafe fn init_moonwave_url_protocol() {
    let ns_url_protocol: &Class = Class::get("NSURLProtocol").unwrap();
    let subclass_name = CString::new("MoonwaveProtocol").unwrap();

    // create class
    let subclass: *mut Class = match Class::get("MoonwaveProtocol") {
        Some(c) => c as *const Class as *mut Class,
        None => {
            let cls = unsafe { objc::runtime::objc_allocateClassPair(ns_url_protocol as *const _, subclass_name.as_ptr(), 0) };
            unsafe { objc::runtime::objc_registerClassPair(cls) };
            cls
        }
    };

    // idk how proper is this but atleast the compiler stfu'ed
    unsafe {
        // canInitWithRequest hook
        let can_init_sel = sel!(canInitWithRequest:);
        let can_init_imp: Imp = std::mem::transmute(can_init_with_request as unsafe extern "C" fn(*mut Class, Sel, *mut Object) -> bool);
        objc::runtime::class_addMethod(
            subclass,
            can_init_sel,
            can_init_imp,
            b"B@:@\0".as_ptr() as *const i8,
        );

        // startLoading hook
        let start_sel = sel!(startLoading);
        let start_imp: Imp = std::mem::transmute(start_loading as unsafe extern "C" fn(*mut Object, Sel));
        objc::runtime::class_addMethod(
            subclass,
            start_sel,
            start_imp,
            b"v@:\0".as_ptr() as *const i8,
        );

        // stopLoading hook (is this even needed?)
        /*let stop_sel = sel!(stopLoading);
        let stop_imp: Imp = std::mem::transmute(stop_loading as unsafe extern "C" fn(*mut Object, Sel));
        objc::runtime::class_addMethod(
            subclass,
            stop_sel,
            stop_imp,
            b"v@:\0".as_ptr() as *const i8,
        );*/
    }

    // registering yay
    let _: () = msg_send![ns_url_protocol, registerClass: subclass];
}

// proper hooks
unsafe extern "C" fn can_init_with_request(_cls: *mut Class, _cmd: Sel, request: *mut Object) -> bool {
    if request.is_null() { return false; }

    let url: *mut Object = msg_send![request, URL];
    if url.is_null() { return false; }

    let abs_url: *mut Object = msg_send![url, absoluteString];
    if abs_url.is_null() { return false; }

    let cstr: *const i8 = msg_send![abs_url, UTF8String];
    if cstr.is_null() { return false; }

    let url_str = unsafe { std::ffi::CStr::from_ptr(cstr).to_string_lossy() };
    const EPIC_DOMAINS: [&str; 6] = [
        "game-social.epicgames.com",
        "ol.epicgames.com",
        "ol.epicgames.net",
        "on.epicgames.com",
        "ak.epicgames.com",
        "epicgames.dev",
    ];
    
    for &domain in EPIC_DOMAINS.iter() {
        if url_str.contains(domain) {
            return true;
        }
    }

    false
}

pub unsafe extern "C" fn start_loading(this: *mut Object, _cmd: Sel) {
    let request: *mut Object = msg_send![this, request];
    if request.is_null() { return; }

    let url: *mut Object = msg_send![request, URL];
    if url.is_null() { return; }

    let backend_cstr = CString::new(BACKEND_URL).unwrap();
    let ns_backend: *mut Object = msg_send![Class::get("NSString").unwrap(), stringWithUTF8String: backend_cstr.as_ptr()];

    let components: *mut Object = msg_send![Class::get("NSURLComponents").unwrap(), componentsWithString: ns_backend];
    let path: *mut Object = msg_send![url, path];
    let query: *mut Object = msg_send![url, query];
    let _: () = msg_send![components, setPath: path];
    let _: () = msg_send![components, setQuery: query];

    let new_url: *mut Object = msg_send![components, URL];
    let _: () = msg_send![request, setURL: new_url];

    let client: *mut Object = msg_send![this, client];
    if !client.is_null() {
        let _: () = msg_send![client,
            URLProtocol:this
            wasRedirectedToRequest:request
            redirectResponse: ptr::null_mut::<Object>()
        ];
    }
}

/*#[no_mangle]
pub unsafe extern "C" fn stop_loading(this: *mut Object, _cmd: Sel) {
    let task: *mut Object = msg_send![this, task];
    if !task.is_null() {
        let _: () = msg_send![task, cancel];
        let _: () = msg_send![this, setTask: ptr::null_mut::<Object>()];
    }
}*/
