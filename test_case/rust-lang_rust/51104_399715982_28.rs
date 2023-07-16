\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gate-trivial_bounds.rs","byte_start":1480,"byte_end":1575,"line_start":69,"line_end":71,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn return_str() -> str where str: Sized { //~ ERROR","highlight_start":1,"highlight_end":52},{"text":"    *\"Sized\".to_string().into_boxed_str()","highlight_start":1,"highlight_end":42},{"text":"}","highlight_start":1,"highlight_end":2}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `str`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"see issue #48214","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"add #![feature(trivial_bounds)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for value values of type `str` cannot be known at compilation time\n  --> /checkout/src/test/ui/feature-gate-trivial_bounds.rs:69:1\n   |\nLL | / fn return_str() -> str where str: Sized { //~ ERROR\nLL | |     *\"Sized\".to_string().into_boxed_str()\nLL | | }\n   | |_^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `str`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized>\n   = help: see issue #48214\n   = help: add #![feature(trivial_bounds)] to the crate attributes to enable\n\n"}
[00:48:08] {"message":"aborting due to 11 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 11 previous errors\n\n"}
[00:48:08] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:48:08] ------------------------------------------
[00:48:08] 
[00:48:08] thread '[ui] ui/feature-gate-trivial_bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:48:08] 
[00:48:08] 
[00:48:08] ---- [ui] ui/mismatched_types/cast-rfc0401.rs stdout ----
[00:48:08] diff of stderr:
[00:48:08] 
[00:48:08] 162 LL |     let _ = v as *const [u8]; //~ ERROR cannot cast
[00:48:08] 164 
[00:48:08] 164 
[00:48:08] - error[E0606]: casting `&Foo` as `*const str` is invalid
[00:48:08] + error[E0606]: casting `&dyn Foo` as `*const str` is invalid
[00:48:08] 167    |
[00:48:08] 167    |
[00:48:08] 168 LL |     let _ = foo as *const str; //~ ERROR is invalid
[00:48:08] 169    |             ^^^^^^^^^^^^^^^^^
[00:48:08] 170 
[00:48:08] 170 
[00:48:08] - error[E0606]: casting `&Foo` as `*mut str` is invalid
[00:48:08] + error[E0606]: casting `&dyn Foo` as `*mut str` is invalid
[00:48:08] 173    |
[00:48:08] 173    |
[00:48:08] 174 LL |     let _ = foo as *mut str; //~ ERROR is invalid
[00:48:08] 200    |
[00:48:08] 200    |
[00:48:08] 201    = help: cast through a thin pointer first
[00:48:08] 202 
[00:48:08] - error[E0606]: casting `*const Foo` as `*const [u16]` is invalid
[00:48:08] + error[E0606]: casting `*const dyn Foo` as `*const [u16]` is invalid
[00:48:08] 205    |
[00:48:08] 205    |
[00:48:08] 206 LL |     let _ = cf as *const [u16]; //~ ERROR is invalid
[00:48:08] 208    |
[00:48:08] 208    |
[00:48:08] 209    = note: vtable kinds may not match
[00:48:08] 210 
[00:48:08] - error[E0606]: casting `*const Foo` as `*const Bar` is invalid
[00:48:08] + error[E0606]: casting `*const dyn Foo` as `*const dyn Bar` is invalid
[00:48:08] 213    |
[00:48:08] 213    |
[00:48:08] 214 LL |     let _ = cf as *const Bar; //~ ERROR is invalid
[00:48:08] 224    |
[00:48:08] 224    |
[00:48:08] 225  j/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary" "-A" "unused"
[00:48:08] ------------------------------------------
[00:48:08] 
[00:48:08] ------------------------------------------
[00:48:08] stderr:
[00:48:08] stderr:
[00:48:08] ------------------------------------------
[00:48:08] {"message":"casting `*const U` as `*const V` is invalid","code":{"code":"E0606","explanation":"\nAn incompatible cast was attempted.\n\nErroneous code example:\n\n