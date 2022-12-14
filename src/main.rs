extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    guess_game();
    number();
    flow();
}

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
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

fn flow() {
    // for item in container == for item in IntoIterator::into_iter(collection) ( 소유권 )
    // for item in &container == for item in collection.iter() -> container를 재사용하고 싶다면 ( 읽기전용 )
    // for item in &mut collection == for item in collection.iter_mut() -> item을 수정해야 하는 경우 ( 읽고 쓰기 )
    let collection = [1,2,3,4,5];
    for i in 0..collection.len() {
        let item =collection[i];
        println!("{}", item);
    }
    // continue
    // while
    // loop
    // break

    // match item {
    //     0 => {},
    //     10 ..= 20 => {},
    //     40 | 80 => {},
    //     _ => {},
    // }
    // swift와 다르게 해당 타입의 가능한 모든 값에 대한 분기를 반드시 처리해야함.

    let haystack = [1,1,2,5,14,42,132,429,1430,4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{} : {}", item, result)
        }
    }


}