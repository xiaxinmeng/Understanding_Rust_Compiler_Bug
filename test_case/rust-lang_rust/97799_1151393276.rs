rust
#![feature(if_let_guard)]
...
match o {
  y if let Some(x) = y => { zzz(x) }
}
