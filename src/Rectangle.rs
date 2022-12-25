#[derive(Debug)]
pub struct Rectangle {
  length: u32,
  width: u32
}

impl Rectangle {
  pub fn area(&self) -> u32 {
    &self.length * &self.width
  }
}

pub fn rectangle(length:u32, width:u32) -> Rectangle {
  let rect = Rectangle { length, width };

  println!(
    "Rectangle area is : {}", rect.area()
  );
  
  rect
}



