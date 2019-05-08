extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub fn console_log<T:AsRef<str>>(s: T){
    // web_sys::console::log(s);
    log(s.as_ref());
}

pub fn console_error<T:AsRef<str>>(s: T){
    // js_sys::Error::new(s);
    error(s.as_ref());
}