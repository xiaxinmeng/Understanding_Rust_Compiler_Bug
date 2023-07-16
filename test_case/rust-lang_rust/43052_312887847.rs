rust
const SZ : usize = 24576;
fn main() {
  let ys: [u64; SZ] = [0; SZ]; // 192KB of space
  println!("Hello world! {} {}", ys[0], ys[SZ-1]);
  println!("Address: {:p} {:p}", &ys[0], &ys[SZ-1]);
}
