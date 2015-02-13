struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

fn sum(x: Vec3, y: Vec3) -> Vec3 {
    Vec3{x: x.x + y.x, y: x.y + y.y, z: x.z + y.z}
}

fn main() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let b = Vec3{x: 4.0, y: 5.0, z: 6.0};
    let c = sum(a, b);
    println!("a+b=({}, {}, {})", c.x, c.y, c.z);
}
