
$ cat empty.rs
#![feature(no_core)]
#![crate_type = "rlib"]
#![no_core]

$ rustup run nightly-2016-04-20 rustc --target stm32f100 --emit=llvm-ir empty.rs
$ head empty.ll
; ModuleID = 'empty.0.rs'
target datalayout = "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv7m-none--eabi"
