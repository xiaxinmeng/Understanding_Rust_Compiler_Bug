
warning: `dyn` is a keyword in the 2018 edition
 --> src/main.rs:7:26
  |
7 | macro_rules! m { () => { dyn Trait } }
  |                          ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
  |
note: lint level defined here
 --> src/main.rs:1:9
  |
1 | #![warn(keyword_idents)]
  |         ^^^^^^^^^^^^^^
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
