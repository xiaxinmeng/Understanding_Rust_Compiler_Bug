rust
pub struct LazyString {
  x: u64,
  cell: OnceCell<String>,
}

impl LazyString {
  pub const fn new(x: u64) -> LazyString { LazyString { x, cell: OnceCell::new() } } 
}

impl Deref for LazyString {
  type Target = String;
  fn deref(&self) -> &String { self.cell.get_or_init(|| self.x.to_string()) }
}
