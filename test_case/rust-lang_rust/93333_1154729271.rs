rust
#![feature(thread_local, asm_sym)]
use std::arch::global_asm;

#[thread_local]
static mut BAR: u64 = 0;

global_asm!(
    r#"
    .globl foo
    foo: mov r8, {}
    "#,
    sym BAR,
);
