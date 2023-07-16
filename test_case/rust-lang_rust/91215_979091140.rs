plain
    Checking rustc-demangle v0.1.21
error[E0308]: mismatched types
    --> library/alloc/src/collections/vec_deque/mod.rs:2187:19
     |
2187 |             if !f(&self[cur]) {
     |                   ^^^^^^^^^^ types differ in mutability
     = note: expected mutable reference `&mut T`
                        found reference `&T`

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> library/alloc/src/collections/vec_deque/mod.rs:2196:19
     |
2196 |             if !f(&self[cur]) {
     |                   ^^^^^^^^^^ types differ in mutability
     = note: expected mutable reference `&mut T`
                        found reference `&T`

For more information about this error, try `rustc --explain E0308`.
