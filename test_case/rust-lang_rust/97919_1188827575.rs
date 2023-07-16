
#[no_mangle]
pub fn greet(name: i32) -> usize {
  aux(name.to_string()).len()
}

fn aux(x: String) -> String {
  format!("Hello, {}", x)
}
