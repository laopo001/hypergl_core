extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

pub fn console_log<T: std::fmt::Debug>(s: T) {
	// web_sys::console::log(s);
	log(&format!("{:?}",s));
}

pub fn console_error<T: std::fmt::Debug>(s: T) {
	// js_sys::Error::new(s);
	error(&format!("{:?}",s));
}