rust
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
  --> opentelemetry-api/src/common.rs:61:9
   |
61 | /         match &self.0 {
62 | |             Cow::Borrowed(s) => s,
63 | |             Cow::Owned(s) => &*s,
64 | |         }
   | |_________^
