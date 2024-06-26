fn main() {
    println!("--- 1.3 ---");
    println!("Hello, world!");

    println!("--- 1.4 ---");
    let x: i32 = 10;
    let y = 20;
    let z = mul(x, y);

    println!("z = {z}");
}


fn mul(x: i32, y: i32) -> i32 {
    x * y
}