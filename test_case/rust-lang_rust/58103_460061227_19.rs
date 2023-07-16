\n\nSee [RFC 911] for more details on the design of `const fn`s.\n\n[RFC 911]: https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-39559-2.rs","byte_start":219,"byte_end":230,"line_start":17,"line_end":17,"column_start":15,"column_end":26,"is_primary":true,"text":[{"text":"        = [0; Dim3::dim()];","highlight_start":15,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants\n  --> /checkout/src/test/ui/issues/issue-39559-2.rs:17:15\n   |\nLL |         = [0; Dim3::dim()];\n   |               ^^^^^^^^^^^\n\n"}
[01:00:58] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-39559-2.rs","byte_start":219,"byte_end":230,"line_start":17,"line_end":17,"column_start":15,"column_end":26,"is_primary":true,"text":[{"text":"        = [0; Dim3::dim()];","highlight_start":15,"highlight_end":26}],"label":"calling non-const function `<Dim3 as Dim>::dim`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/issues/issue-39559-2.rs:17:15\n   |\nLL |         = [0; Dim3::dim()];\n   |               ^^^^^^^^^^^ calling non-const function `<Dim3 as Dim>::dim`\n\n"}
[01:00:58] {"message":"For more information about this error, try `rustc --explain E0015`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0015`.\n"}
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] 
---
[01:00:58] 
[01:00:58] 6    |
[01:00:58] 7    = note: an implementation of `std::ops::Add` might be missing for `&str`
[01:00:58] 8 
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] + error: any use of this value will cause an error
[01:00:58] 11    |
[01:00:58] 11    |
[01:00:58] 12 LL |     A = Foo::A as isize
[01:00:58] 
[01:00:58] 13    |         ^^^^^^^^^^^^^^^ referenced constant has errors
[01:00:58] +    = note: #[deny(const_err)] on by default
[01:00:58] 14 
[01:00:58] 15 error: aborting due to 2 previous errors
[01:00:58] 16 
[01:00:58] 16 
[01:00:58] 
[upport it.\nErroneous code example:\n\n