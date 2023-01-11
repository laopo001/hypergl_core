use std::marker::PhantomPinned;
use std::ptr::NonNull;
struct A {
    name: String,
}
impl A {
    fn new(name: &str) -> Self {
        return A {
            name: name.to_string(),
        };
    }
}

async fn test() -> A {
    let mut a = A::new("name");
    println!("{:p}", &a.name);

    return a;
}

async fn run() -> A {
    let mut a = test().await;
    println!("{:p}", &a.name);

    return a;
}

fn main() {
    pollster::block_on(run());
}
