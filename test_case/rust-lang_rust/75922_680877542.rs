Rust
#![feature(asm)]
#![feature(naked_functions)]

#[inine(never)]
// Removing #[naked] allows this to compile.
#[naked]
fn bar(ptr: *mut u64) {
  unsafe {
    asm!(
     "mov {foo}, {foo}",
     foo = inout(reg) *ptr,
    )
  }
}

fn main() {
  bar(&mut 0);
}
