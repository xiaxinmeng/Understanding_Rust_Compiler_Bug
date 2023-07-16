
   | |_______^
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/check-attr.rs:1:9
   |
LL | #![deny(rustdoc::invalid_codeblock_attributes)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: the code block will either not be tested if not marked as a rust one or won't fail if it compiles successfully

error: unknown attribute `compilefail`. Did you mean `compile_fail`?
   |
   |
LL | / /// foo
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 