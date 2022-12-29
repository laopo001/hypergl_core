use std::ptr::NonNull;

struct A {
    name: String,
}

async fn test() -> A {
    let mut a = A {
        name: "aaa".to_string(),
    };
    println!("{:p}", &a);

    return a;
}

async fn run() -> A {
    let mut a = test().await;
    println!("{:p}", &a);

    return a;
}

fn main() {
    pollster::block_on(run());
}
