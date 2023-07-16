rust
#[naked]
fn hehehe() -> ! {
  loop {} // No inline asm here!
}
