#![feature(box_into_raw_non_null, box_syntax, box_patterns)]

#[allow(unused_mut)]
#[macro_use]
extern crate lazy_static;
extern crate cfg_if;
pub mod application;
pub mod graphics;
pub mod config;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
