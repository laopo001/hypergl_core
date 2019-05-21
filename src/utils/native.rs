pub fn console_log<T:std::fmt::Debug>(s: T){
    println!("{:?}",s);
}

pub fn console_error<T:AsRef<str> + std::fmt::Debug>(s: T){
    println!("{:?}",s.as_ref());
}