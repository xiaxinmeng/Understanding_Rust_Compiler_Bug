console
warning: path statement drops value
 --> a.rs:4:5
  |
4 |     p;
  |     ^^ help: use `drop` to clarify the intent: `drop(p);`
  |
  = note: `#[warn(path_statements)]` on by default

warning: path statement with no effect
 --> a.rs:5:5
  |
5 |     i;
  |     ^^

warning: 2 warnings emitted
