 rust
#![feature(asm)]

fn main() {
  let x = 23i;
  unsafe {
    asm!(
      "
      mov $$60, %rax
      mov $0, %rdi
      syscall
      "
      :
      : "r"(x)
      : "rax", "rdi"
    );
  }
}
