
bug-21221.rs:22:6: 22:9 error: use of undeclared trait name `Mul`. 
Are you looking for `mul1::Mul` or `mul2::Mul` from this crate? 
Or maybe `Mul` from another crate? [E0405]
bug-21221.rs:22 impl Mul for Foo { 
                     ^~~
bug-21221.rs:22:6: 22:9 help: run `rustc --explain E0405` to see a
detailed explanation
