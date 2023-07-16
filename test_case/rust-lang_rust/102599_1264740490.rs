plain
---- [rustdoc] src/test/rustdoc/inline_local/please_inline.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/please_inline/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/please_inline" "--deny" "warnings" "/checkout/src/test/rustdoc/inline_local/please_inline.rs"
stdout: none
--- stderr -------------------------------
error: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
17 |     #[feature(inline)]
   |
   |
   = note: `-D unused-attributes` implied by `-D warnings`
error: aborting due to previous error
------------------------------------------


