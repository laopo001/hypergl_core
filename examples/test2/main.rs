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
    dbg!(nalgebra::Matrix4::<f32>::new_perspective(
        1.0,
        0.25 * std::f32::consts::PI,
        0.1,
        100.0,
    ));

    dbg!(glam::Mat4::perspective_rh(
        1.0,
        0.25 * std::f32::consts::PI,
        0.1,
        100.0
    ));
    // let m = glam::Mat4::look_at_rh(
    //     glam::Vec3::new(2., 2., 2.),
    //     glam::Vec3::new(0., 0., 0.),
    //     glam::Vec3::new(0., 1., 0.),
    // );
    // dbg!(&m);
    // let q = glam::Quat::from_mat4(&m.inverse());
    // dbg!(&q);
    // dbg!(glam::Mat4::from_scale_rotation_translation(
    //     glam::Vec3::new(1.0, 1.0, 1.0),
    //     q,
    //     glam::Vec3::new(2.0, 2.0, 2.0),
    // ));
    // dbg!(glam::Mat4::from_scale_rotation_translation(
    //     glam::Vec3::new(1.0, 1.0, 1.0),
    //     q,
    //     glam::Vec3::new(2.0, 2.0, 2.0),
    // )
    // .inverse());

    // pollster::block_on(run());
}
