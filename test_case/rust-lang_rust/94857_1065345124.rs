plain
warning: `tidy` is not installed; diffs will not be generated

running 509 tests
i.....................................................i............................................. 100/509
....................................................................F....F.......................F.. 200/509
........................................i........................................................... 400/509
.........i.......................................................................................... 500/509
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........
.........
failures:

---- [rustdoc] rustdoc/intra-doc/cross-crate/module.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/module/auxiliary/module/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/module" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/cross-crate/auxiliary/module.rs"
stdout: none
--- stderr -------------------------------
no resolution for "SomeTrait" MacroNS DefId(0:0 ~ module_inner[7bba])
no resolution for "SomeTrait" TypeNS DefId(0:0 ~ module_inner[7bba])
no resolution for "SomeTrait" ValueNS DefId(0:0 ~ module_inner[7bba])
error: unresolved link to `SomeTrait`
 --> /checkout/src/test/rustdoc/intra-doc/cross-crate/auxiliary/module.rs:6:21
  |
6 | /// [bar] links to [SomeTrait] and also [SomeType]
  |                     ^^^^^^^^^ no item named `SomeTrait` in scope
note: the lint level is defined here
 --> /checkout/src/test/rustdoc/intra-doc/cross-crate/auxiliary/module.rs:2:9
  |
  |
2 | #![deny(rustdoc::broken_intra_doc_links)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] rustdoc/intra-doc/enum-struct-field.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/enum-struct-field/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/enum-struct-field" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/enum-struct-field.rs"
stdout: none
--- stderr -------------------------------
no resolution for "Foo" TypeNS DefId(0:0 ~ foo[fddf])
error: unresolved link to `Foo::X::y`
   |
   |
11 | /// I want [Foo::X::y].
   |             ^^^^^^^^^ `X` is a variant, not a module or type, and cannot have associated items
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: aborting due to previous error
------------------------------------------



---- [rustdoc] rustdoc/intra-doc/mod-relative.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/mod-relative/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/mod-relative" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/mod-relative.rs"
stdout: none
--- stderr -------------------------------
no resolution for "Test::do_test" MacroNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test::do_test" TypeNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test" TypeNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test::do_test" ValueNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test" TypeNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test" TypeNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test" ValueNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
no resolution for "Test" MacroNS DefId(0:3 ~ mod_relative[4c81]::wrapper)
error: unresolved link to `Test::do_test`
   |
   |
13 |     /// [`Test::do_test`]
   |           ^^^^^^^^^^^^^ no item named `Test` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: aborting due to previous error
------------------------------------------


