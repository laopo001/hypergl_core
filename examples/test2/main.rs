use handlebars::Handlebars;
use std::collections::HashMap;

fn main() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", include_str!("./templates/index.txt"))
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Rust");

    println!("{}", handlebars.render("hello", &data).unwrap());
}
