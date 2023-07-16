
error[E0015]: cannot perform deref coercion on `String` in constant functions
    --> src/lib.rs:7:9
     |
7    | /         match &self.0 {
8    | |             Cow::Borrowed(s) => s,
9    | |             Cow::Owned(s) => &*s,
10   | |         }
     | |_________^
     |
     = note: attempting to deref into `str`
note: impl defined here, but it is not `const`
     = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
