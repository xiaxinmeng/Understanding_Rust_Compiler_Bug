
[root@li1424-173 gh35849]# build/x86_64-unknown-linux-gnu/stage1/bin/rustc - -o bad <<<'fn foo() -> ! { 42 } fn main() { foo(); }'
warning: broken MIR (return = const 42i32): bad assignment (! = i32): Sorts(ExpectedFound { expected: !, found: i32 })
 --> <anon>:1:17
  |
1 | fn foo() -> ! { 42 } fn main() { foo(); }
  |                 ^^

[root@li1424-173 gh35849]# ./bad
Segmentation fault (core dumped)
