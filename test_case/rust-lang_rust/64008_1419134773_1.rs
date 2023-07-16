text
Compiling playground v0.0.1 (/playground)
warning: unused variable: `get`
 --> src/main.rs:2:9
  |
2 |     let get = |path: &str| || {
  |         ^^^ help: if this is intentional, prefix it with an underscore: `_get`
  |
  = note: `#[warn(unused_variables)]` on by default

error: lifetime may not live long enough
 --> src/main.rs:2:28
  |
2 |       let get = |path: &str| || {
  |  ______________________-___-_^
  | |                      |   |
  | |                      |   return type of closure `[closure@src/main.rs:2:28: 2:30]` contains a lifetime `'2`
  | |                      let's call the lifetime of this reference `'1`
3 | |         assert!(path.starts_with('/'));
4 | |     };
  | |_____^ returning this value requires that `'1` must outlive `'2`
  |
help: consider adding 'move' keyword before the nested closure
  |
2 |     let get = |path: &str| move || {
  |                            ++++

warning: `playground` (bin "playground") generated 1 warning
error: could not compile `playground` due to previous error; 1 warning emitted
