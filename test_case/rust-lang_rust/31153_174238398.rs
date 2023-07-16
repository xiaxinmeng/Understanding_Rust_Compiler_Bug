 rust
let tuple: (u8, String) = (...);

let (x, _) = tuple;

match tuple {
     (0, _) => { ... }
     (1, ref y) => { ... }
     _ => { ... }
}
