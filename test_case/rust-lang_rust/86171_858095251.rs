console
$ rustc main.rs -Cpasses=lint
Undefined behavior: Buffer overflow
  %9 = load { i64, i32 }, { i64, i32 }* %8, align 4
