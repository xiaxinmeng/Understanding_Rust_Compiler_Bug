rust
fn bar<'a>(x: &'static str, y: &'a str, z: &'a str) -> impl Future<Output=()> + 'a {
  async move {
    println!("{} {}", y, z);
  }
}
