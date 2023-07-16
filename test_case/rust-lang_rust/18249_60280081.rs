
fn main() {
  std::os::args().as_slice().head().map(Clone::clone);
}
