rust
struct try { a: Option<u32> }

let a = Some(1);
let b = try { a }; // is this a `try` expression or a struct literal?
