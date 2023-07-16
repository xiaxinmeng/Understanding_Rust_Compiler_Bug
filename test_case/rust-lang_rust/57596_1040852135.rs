rust
pub fn error(msg: String) -> Box<dyn std::error::Error> {
  msg.into() 
}

fn main() {
  error(String::new());
}
