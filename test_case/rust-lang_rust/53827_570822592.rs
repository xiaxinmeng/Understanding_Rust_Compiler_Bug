rust
let mut a = vec![-1; 2048 * 2048].into_boxed_slice();

fn get_item(x: usize, y: usize) -> i32 {
  a[x % 2048 + y / 2048] // a[x][y]
}
