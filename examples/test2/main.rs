#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

use std::error::Error;
use tera::{Context, Result, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("examples/test2/templates/**.txt") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.add_raw_template("hello.html", include_str!("./templates/index.txt"))
        //     .unwrap();
        tera.autoescape_on(vec![".txt"]);

        tera
    };
}

fn main() {
    let mut context = Context::new();
    context.insert("username", &"Bob");

    match TEMPLATES.render("index.txt", &context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("Error: {}", e);
            let mut cause = e.source();
            while let Some(e) = cause {
                println!("Reason: {}", e);
                cause = e.source();
            }
        }
    };
}
