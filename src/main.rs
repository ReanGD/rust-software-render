use std::cmp::Ordering;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

fn sum(x: Vec3, y: Vec3) -> Vec3 {
    Vec3{x: x.x + y.x, y: x.y + y.y, z: x.z + y.z}
}

fn cmp(x: i32, y: i32) -> Ordering {
    if x > y { Ordering::Greater }
    else if x == y { Ordering::Equal }
    else { Ordering::Less }
}

fn print_ordering(v: Ordering) {
    if v == Ordering::Greater { println!("Greater"); }
    else if v == Ordering::Equal { println!("Equal"); }
    else { println!("Less"); }
}

fn main() {
    let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let b = Vec3{x: 4.0, y: 5.0, z: 6.0};
    let c = sum(a, b);
    println!("a+b=({}, {}, {})", c.x, c.y, c.z);

    let x = 5;
    let y = 7;
    let r = cmp(x, y);
    print_ordering(r);
}
