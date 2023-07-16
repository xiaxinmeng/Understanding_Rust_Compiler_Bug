\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/uninhabited_enum_discriminant1.rs","byte_start":612,"byte_end":625,"line_start":14,"line_end":14,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"    A = X::A as isize, //~ ERROR enums without inhabited variants do not have discriminants","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0080]: constant evaluation of enum discriminant resulted in non-integer\essfully in 0:02:02
[00:41:18] Makefile:58: recipe for target 'check' failed
[00:41:18] make: *** [check] Error 1
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs
