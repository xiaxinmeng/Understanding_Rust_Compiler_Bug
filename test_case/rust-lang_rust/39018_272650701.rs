
michael@eva-02 ~/Code/rustc (git)-[better-string-message] % ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc string.rs
_
error[E0369]: binary operation `+` cannot be applied to type `&str`
 --> string.rs:4:20
  |
4 |     println!("{}", s1.as_str() + s2.as_str());
  |                    ^^^^^^^^^^^
  |
note: You can't use `+` between a &str and &str
 --> string.rs:4:20
  |
4 |     println!("{}", s1.as_str() + s2.as_str());
  |                    ^^^^^^^^^^^

error: aborting due to previous error

