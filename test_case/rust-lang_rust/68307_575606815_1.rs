rust
let closure: !Copy = bar()
(move |x: i32| closure(x)) : !Copy
