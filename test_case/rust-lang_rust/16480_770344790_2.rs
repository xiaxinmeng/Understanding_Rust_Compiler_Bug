
$ rustc main.rs
warning: unused variable: `a`
 --> main.rs:1:21
  |
}   let a = 0;{
  |                    ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted
