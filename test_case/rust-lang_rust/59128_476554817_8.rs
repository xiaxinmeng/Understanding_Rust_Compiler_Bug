
[01:51:12]    |
[01:51:12]    = note: error from rustc: unknown start of token: \
[01:51:12] help: mark blocks that do not contain Rust code as text
[01:51:12]    |
---
[01:51:12] 
[01:51:12] warning: doc comment contains an invalid Rust code block
[01:51:12]   --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:63:1
[01:51:12]    |
[01:51:12] LL | / #[doc = "