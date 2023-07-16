
$ cat 38785.rs
#![feature(asm)]


struct CPUTableDescriptor {
    limit: u16,
    base: u64,
}


fn lidt(idt_desc : CPUTableDescriptor) {
    unsafe {
        asm!(
            "lidt $0"
            :
            : "m"(idt_desc)
            :
            );
    }
}

fn main() { }

$ rustc +nightly -vV
rustc 1.30.0-nightly (2f1547c0a 2018-09-11)
binary: rustc
commit-hash: 2f1547c0aa5957b42cc768c00119c6eb7b4262d3
commit-date: 2018-09-11
host: x86_64-unknown-linux-gnu
release: 1.30.0-nightly
LLVM version: 8.0
$ rustc +nightly 38785.rs 
warning: struct is never constructed: `CPUTableDescriptor`
 --> 38785.rs:4:1
  |
4 | struct CPUTableDescriptor {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `lidt`
  --> 38785.rs:10:1
   |
10 | fn lidt(idt_desc : CPUTableDescriptor) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

$ echo $status
0
