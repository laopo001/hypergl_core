#[cfg(all(target_arch = "wasm32"))]
include!("./web.rs");

#[cfg(not(target_arch = "wasm32"))]
include!("./native.rs");