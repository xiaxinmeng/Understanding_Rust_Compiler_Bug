console
$ rustc +nightly-2021-08-14 -Cno-prepopulate-passes --emit=llvm-ir src/main.rs -o before.ll
$ opt before.ll  # ok

$ rustc +nightly-2021-08-15 -Cno-prepopulate-passes --emit=llvm-ir src/main.rs -o after.ll
$ opt after.ll
opt: after.ll:287:1: error: expected instruction opcode
bb1:                                              ; preds = %bb0
^
