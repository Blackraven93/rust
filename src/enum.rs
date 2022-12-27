
fn enumerateFn() {
  enum IpAddressKind {
    v1,
    v2,
    v3,
  }
  
  let one = IpAddressKind::v1;
  let two = IpAddressKind::v2;
  let three = IpAddressKind::v3;

  route(one);
  route(two);
  route(three);
}

fn route(ip_type: IpAddressKind) {

  struct IpAddress {
    kine: IpAddressKind,
    address: String,
  }

  let home = IpAddress {
    kind: ip_type,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddress {
    kind: IpAddrKind::v3,
    address: String::from("::1"),
  };

  // enum IpAddr {
  //   V4(String),
  //   V6(String),
  // }

  // let home = IpAddr::V4(String::from("127.0.0.1"));

  // let loopback = IpAddr::V6(String::from("::1"));

  // enum IpAddr {
  //     V4(u8, u8, u8, u8),
  //     V6(String),
  // }

  // let home = IpAddr::V4(127, 0, 0, 1);

  // let loopback = IpAddr::V6(String::from("::1"));
}

fn message() {
  struct QuitMessage; 
  struct MoveMessage {
      x: i32,
      y: i32,
  }
  struct WriteMessage(String);
  struct ChangeColorMessage(i32, i32, i32);

  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChnageColor(i32, i32, i32)
  }

  impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
  } 

  let m = Message::Write(String::from("hello"));
  m.call();

  enum Option<T> {
    // Some<T>,
    None,
  }

  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;

  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum = x + y;
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}