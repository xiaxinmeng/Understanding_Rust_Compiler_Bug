\n\nSee [RFC 1522] for more details.\n\n[RFC 1522]: https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":7484,"byte_end":7494,"line_start":224,"line_end":224,"column_start":22,"column_end":32,"is_primary":true,"text":[{"text":"    where T: Fn() -> impl Debug","highlight_start":22,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(impl_trait_in_bindings)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0562]: `impl Trait` not allowed outside of function and inherent method return types\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:224:22\n   |\nLL |     where T: Fn() -> impl Debug\n   |                      ^^^^^^^^^^\n   |\n   = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable\n\n"}
[00:52:33] {"message":"`impl Trait` not allowed outside of function and inherent method return types","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` fction and inherent method return types
[00:52:33] 
[00:52:33] 9    |
[00:52:33] 9    |
[00:52:33] 10 LL | enum Enum<T: Iterable<Item = impl Foo>> {
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 12 
[00:52:33] 12 
[00:52:33] 13 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 
[00:52:33] 15    |
[00:52:33] 15    |
[00:52:33] 16 LL | union Union<T: Iterable<Item = impl Foo> + Copy> {
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 18 
[00:52:33] 18 
[00:52:33] 19 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 
[00:52:33] 21    |
[00:52:33] 21    |
[00:52:33] 22 LL | type Type<T: Iterable<Item = impl Foo>> = T;
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 24 
[00:52:33] 25 error: aborting due to 4 previous errors
[00:52:33] 25 error: aborting due to 4 previous errors
[00:52:33] 26 
[00:52:33] 
[00:52:33] 
[00:52:33] The actual stderr differed from the expected stderr.
[00:52:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47715/issue-47715.stderr
[00:52:33] To update references, rerun the tests and pass the `--bless` flag
[00:52:33] To only{\n    for i in count_to_n(10) {  // ok!\n        println!(\"{}\", i);\n    }\n}\n