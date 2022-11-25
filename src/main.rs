fn main() {
    number()
}

fn number() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let _d = 30_i32;
    let e = add(add(a,b), add(c,b));

    println!("(a + b) + (c + d) = {}", e);

    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0, 42f32, 42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);

    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    // i8, i16, i32, o64, i128 8 ~ 128비트 크기의 부호 있는 정수
    // u8, u16, u32, u64, u128 8 ~ 128비트 크기의 부호 없는 정수
    // f32, f64 32비트, 64비트 종류가 있는 부동 소수점 수
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}