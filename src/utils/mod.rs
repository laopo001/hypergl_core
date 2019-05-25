#[cfg(all(target_arch = "wasm32"))]
include!("./web.rs");

#[cfg(not(target_arch = "wasm32"))]
include!("./native.rs");

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