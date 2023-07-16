text
   = note: error from rustc: unknown start of token: \

---

warning: could not parse code block as Rust code
  --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:92:9
   |
LL |   ///     \____/
LL | | ///
   | |_
   |
   = note: error from rustc: unknown start of token: \
   = note: error from rustc: unknown start of token: \

warning: could not parse code block as Rust code
  --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:97:5
   |
LL |   /// 