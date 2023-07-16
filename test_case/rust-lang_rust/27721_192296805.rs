 rust
let_scan!("before:after", (let word_0 <| until(":"), ":", let word_1: Everything));
assert_eq!(word_0, "before");
assert_eq!(word_1, "after");
