fn main() {
    println!("--- 1.3 ---");
    println!("Hello, world!");

    println!("--- 1.4 ---");
    let x: i32 = 10;
    let y = 20;
    let z = mul(x, y);

    println!("z = {z}");

    println!("--- 2.1 ---");
    println!("---- 短絡評価 論理和 [ a() || b() ]----");
    println!("{}", a() || b());

    println!("---- 非短絡評価 論理和 [ a() | b() ] -----");
    println!("{}", a() | b());
    
	
	println!("--- 2.1 ---");
	let n: u8 = 0b0001_1000;
	println!("n      = 0b{:08b}", n);
	let m = n << 2;
	println!("n << 2 = 0b{:08b}", m);
	let k = n >> 2;
	println!("n >> 2 = 0b{:08b}", k);

}


fn mul(x: i32, y: i32) -> i32 {
    x * y
}

// --- chapter2 ---
fn a() -> bool {
    println!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}

