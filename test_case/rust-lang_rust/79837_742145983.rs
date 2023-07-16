rust
#![feature(asm)]

fn main() {
    let mut value: u64;
    unsafe {
        asm!(
          "mov {value}, 1",
          "lea rax, 3f[rip]",
          "jmp rax",
          "mov {value}, 2",
          "3:",
          value = out(reg) value,
          out("rax") _,
        );
    }
    dbg!(value);
}
