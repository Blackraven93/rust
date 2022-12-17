# 변수와 가변성

- let
- mut

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

```rust
  // Error
  let mut spaces = "   ";
  spaces = spaces.len();

  // Correct ( shadowing ) : 차이는 다음 정의한 let은 새로 정의된 값이다.
  let spaces = "   ";
  let spaces = spaces.len();
```

새로 할당

- const

```rust
  const MAX_POINTS: u32 = 100_000;
```

## Type

- 32-bit Int

  - Signed: i32
  - Unsigned: u32

- bool
- char
- tuple

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

- array
  - array elements are same type
  - When do u use array?
    - 당신의 데이터를 heap보다 stack에 할당하는 것을 원할 때
    - 항상 고정된 숫자의 요소를 갖는다고 확신하고 싶을 때

# 함수

- 함수 동작 시 표현식과 구문에 대한 이해 필요
- 세미콜론을 이용하여 return 값 유무 확인 (표현식과 구문 분류)

# OwnerShip(소유권)

## 소유권

- stack
- heep

- shallow copy
- deep copy

## 참조자

## 슬라이스
