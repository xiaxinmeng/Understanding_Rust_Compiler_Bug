
{"message":"`#[c_ffi_const]` function cannot be`#[c_ffi_pure]`","code":{"code":"E0726","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/c_ffi_const2.rs","byte_start":77,"byte_end":90,"line_start":5,"line_end":5,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    #[c_ffi_pure] //~ ERROR `#[c_ffi_const]` function cannot be`#[c_ffi_pure]` [E0726]","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0726]: `#[c_ffi_const]` function cannot be`#[c_ffi_pure]`\n  --> /checkout/src/test/ui/c_ffi_const2.rs:5:5\n   |\nLL |     #[c_ffi_pure] //~ ERROR `#[c_ffi_const]` function cannot be`#[c_ffi_pure]` [E0726]\n   |     ^^^^^^^^^^^^^\n\n"}
[01:05:37] Attributes 'readnone and readonly' are incompatible!
[01:05:37] void ()* @baz
[01:05:37] LLVM ERROR: Broken module found, compilation aborted!
