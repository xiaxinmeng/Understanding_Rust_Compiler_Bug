
   |
   = note: error from rustc: unknown start of token: `
   = note: error from rustc: unknown start of token: `
help: mark blocks that do not contain Rust code as text
---

warning: could not parse code block as Rust code
  --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:45:9
   |
LL |   ///     code with bad syntax
LL | | ///     \_
   | |__________^
   |
   = note: error from rustc: unknown start of token: \
---

warning: doc comment contains an invalid Rust code block
  --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:70:1
   |
LL | / #[doc = "