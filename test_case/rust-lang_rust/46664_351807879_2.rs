\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/inference-variable-behind-raw-pointer.rs","byte_start":563,"byte_end":570,"line_start":14,"line_end":14,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"    if data.is_null() {}","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"this will be made into a hard error in a future version of the compiler","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0619]: the type of this value must be known in this context\n  --> /checkout/src/test/ui/inference-variable-behind-raw-pointer.rs:14:13\n   |\n14 |     if data.is_null() {}\n   |             ^^^^^^^\n   |\n   = note: this will be made into a hard error in a future version of the compiler\n\n"}
[00:53:29] 
[00:53:29] ------------------------------------------
[00:53:29] 
[00:53:29] thread '[ui] ui/inference-variable-behind-raw-pointer.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2774:8
[00:53:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:29] 
[00:53:29] 
[00:53:29] failures:
[00:53:29]     [ui] ui/inference-variable-behind-raw-pointer.rs
[00:53:29] 
[00:53:29] test result: FAILED. 666 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
