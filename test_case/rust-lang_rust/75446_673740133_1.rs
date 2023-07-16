rust
#!/usr/bin/env run-cargo-script

fn main() -> std::io::Result<()> {
  println!("{}", std::fs::copy("./overlay/IN", "./direct/OUT")?);
  Ok(())
}
