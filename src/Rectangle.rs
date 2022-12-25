pub struct Rectangle {
  width: u32,
  height: u32
}

pub fn rectangle(width:u32, height:u32) -> Rectangle{
  let rect = Rectangle { width, height };

  println!(
    "Rectangle area is : {}", area((rect.width, rect.height))
  );

  rect
}

fn area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

