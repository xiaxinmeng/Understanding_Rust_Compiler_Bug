
$ touch empty.c
$ clang --target=thumbv7m-none-eabi -S -emit-llvm empty.c -o empty.ll
$ head empty.ll
; ModuleID = 'empty.c'
target datalayout = "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv7m-none--eabi"
