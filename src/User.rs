struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

pub fn user() {  
  
  let user1 = create_user("blackraven@gmail.com".to_owned(), "raven".to_owned());
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  //ERROR: missing lifetime specifier
  // struct User2 { 
  //   username: &str,
  //   email: &str,
  //   sign_in_count: u64,
  //   active: bool,
  // }

  // let user3 = User2 {
  //       email: "someone@example.com",
  //       username: "someusername123",
  //       active: true,
  //       sign_in_count: 1,
  //   };
}

fn create_user(email:String, username:String)  -> User {
   User {
    email,
    username,
    sign_in_count: 1,
    active: true,
  }
}
