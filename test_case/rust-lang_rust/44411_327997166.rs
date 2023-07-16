text
$ cat foo.c
#include <stdbool.h>
const bool FOO = true;

$ clang --target=s390x-unknown-linux-gnu -S -emit-llvm -std=c11 foo.c -o -
; ModuleID = 'foo.c'
source_filename = "foo.c"
target datalayout = "E-m:e-i1:8:16-i8:8:16-i64:64-f128:64-a:8:16-n32:64"
target triple = "s390x-unknown-linux-gnu"

@FOO = constant i8 1, align 2

!llvm.ident = !{!0}

!0 = !{!"clang version 4.0.1 (tags/RELEASE_401/final)"}
