rust
fn is_the_optimizer_on() -> bool {
  unsafe {
    asm!("mov r0, 1":::"volatile")
    asm!("mov r0, 0":::)
    let x: usize;
    asm!("mov %0, r0"::"=r"(x):);
    x == 1
  }
}
