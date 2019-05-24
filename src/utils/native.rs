pub fn console_log<T: std::fmt::Debug>(s: T) {
	println!("{:?}", s);
}

pub fn console_error<T: std::fmt::Debug>(s: T) {
	println!("{:?}", s);
	panic!("console error");
}