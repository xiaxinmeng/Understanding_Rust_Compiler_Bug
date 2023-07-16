rust
const SIZE: usize = 2048;

let mut a = vec![-1; SIZE * SIZE].into_boxed_slice();

fn get_item(x: usize, y: usize) -> i32 {
  a[x % SIZE + y / SIZE] // a[x][y]
}
