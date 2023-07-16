rust
use staticvec::{StaticString, StringError};

#[derive(Debug)]
pub struct User {
  pub username: StaticString<20>,
  pub role: StaticString<5>,
}

fn main() -> Result<(), StringError> {
  let user = User {
    username: StaticString::try_from_str("user")?,
    role: StaticString::try_from_str("admin")?,
  };
  println!("{:?}", user);
  Ok(())
}
