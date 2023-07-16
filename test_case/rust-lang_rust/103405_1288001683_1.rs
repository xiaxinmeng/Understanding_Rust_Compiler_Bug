console
warning: irrefutable `if let` pattern
  --> /home/cat/code/rust/src/test/ui/parser/issue-103381.rs:13:19
   |
13 |     if true && if let x = 1 { true } else { true } {}
   |                   ^^^^^^^^^
   |
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: this pattern will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`

warning: 1 warning emitted
