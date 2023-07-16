
$ opt -disable-output -lint foreign-truncated-arguments.ll
Undefined behavior: Buffer overflow
  %4 = load { i64, i32 }, { i64, i32 }* %3, align 4
