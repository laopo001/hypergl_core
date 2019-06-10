//#[cfg(all(target_arch = "wasm32"))]
//include!("./web.rs");
//
//#[cfg(not(target_arch = "wasm32"))]
//include!("./native.rs");


pub mod console {
	#[cfg(all(target_arch = "wasm32"))]
	use wasm_bindgen::prelude::*;
	#[cfg(all(target_arch = "wasm32"))]
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_namespace = console)]
		fn web_log(s: &str);
		#[wasm_bindgen(js_namespace = console)]
		fn web_error(s: &str);
	}


	#[cfg(all(target_arch = "wasm32"))]
	pub fn log<T: std::fmt::Debug>(s: T) {
		web_log(&format!("{:?}",s));
	}
	#[cfg(all(target_arch = "wasm32"))]
	pub fn error<T: std::fmt::Debug>(s: T) {
		web_error(&format!("{:?}",s));
	}
	#[cfg(not(target_arch = "wasm32"))]
	pub fn log<T: std::fmt::Debug>(s: T) {
		println!("{:?}", s);
	}
	#[cfg(not(target_arch = "wasm32"))]
	pub fn error<T: std::fmt::Debug>(s: T) {
		println!("{:?}", s);
		panic!("console error");
	}
}

pub struct Time {}

impl Time {
	#[cfg(not(target_arch = "wasm32"))]
	pub fn now() -> u128 {
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
	}
	#[cfg(all(target_arch = "wasm32"))]
	pub fn now() -> u128 {
		let d = js_sys::Date::new_0();
		d.get_time() as u128
	}

}