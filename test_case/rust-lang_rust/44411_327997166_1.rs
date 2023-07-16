text
$ cat foo.rs
pub static FOO: bool = true;

$ rustc --target=s390x-unknown-linux-gnu --crate-type=rlib --emit=llvm-ir foo.rs

$ cat foo.ll
; ModuleID = 'foo.cgu-0.rs'
source_filename = "foo.cgu-0.rs"
target datalayout = "E-m:e-i1:8:16-i8:8:16-i64:64-f128:64-a:8:16-n32:64"
target triple = "s390x-unknown-linux-gnu"

@_ZN3foo3FOO17h5b2ce89c23349598E = constant i8 1, align 1
