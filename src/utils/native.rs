pub fn console_log<T:AsRef<str> + std::fmt::Debug>(s: T){
    println!("{:?}",s.as_ref());
}

pub fn console_error<T:AsRef<str> + std::fmt::Debug>(s: T){
    println!("{:?}",s.as_ref());
}