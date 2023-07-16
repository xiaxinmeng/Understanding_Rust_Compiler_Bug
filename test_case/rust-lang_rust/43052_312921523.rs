
$ echo 'fn main() { main() }' | rustc -
warning: function cannot return without recurring
 --> <anon>:1:1
  |
1 | fn main() { main() }
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unconditional_recursion)] on by default
note: recursive call site
 --> <anon>:1:13
  |
1 | fn main() { main() }
  |             ^^^^^^
  = help: a `loop` may express intention better if this is on purpose

$ ./rust_out

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
Aborted
