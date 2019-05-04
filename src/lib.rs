#[allow(unused_mut)]
#[macro_use]
extern crate lazy_static;
pub mod application;
pub mod graphics;
pub mod config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
