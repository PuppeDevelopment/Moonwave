#![allow(unused)]

pub const BACKEND_URL: &str = "http://192.168.1.69:8080"; // Set valid IP, not 127.0.0.1 for mobile
pub const USE_CURL_SYMBOL: bool = true; // disable this if 18.40+
pub const USE_EOS: bool = false; // enable this if 22+
pub const BYPASS_SSL: bool = false; // idk why u will use this its js for debugging with fiddler

// Partyhub shit
pub const LOGIN_URL: &str = "http://192.168.1.69:5000/login";
pub const REGISTER_URL: &str = "http://192.168.1.69:5000/register";