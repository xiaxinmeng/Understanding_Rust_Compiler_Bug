 rust
fn copy_file(src: Path, dst: Path) -> bool {
  if dst.exists() { return false; }
  if !src.is_file() { return false; }
  // ... copy file
}
