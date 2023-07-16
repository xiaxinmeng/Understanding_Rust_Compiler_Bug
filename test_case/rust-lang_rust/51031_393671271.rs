
error[E0594]: cannot assign to immutable item `x`
  --> $DIR/issue-45983.rs:17:18
   |
LL |     give_any(|y| x = Some(y));
   |                  ^^^^^^^^^^^ cannot mutate
   |
   = note: the value which is causing this path not to be mutable is...: `x`
