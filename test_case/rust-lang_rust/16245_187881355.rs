 rust
fn bool_to_i32(val: bool) -> i32 {
  (match val {
    true => 1i32,
    false => 0i32
  }) as i32
}

fn main() {}
