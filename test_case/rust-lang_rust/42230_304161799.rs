bash
$ build/x86_64-unknown-linux-gnu/stage2/bin/rustc src/test/compile-fail/issue-37665.rs \
-L build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json \
-L build/x86_64-unknown-linux-gnu/test/compile-fail/issue-37665.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux \
-C prefer-dynamic -o build/x86_64-unknown-linux-gnu/test/compile-fail/issue-37665.stage2-x86_64-unknown-linux-gnu \
-Crpath -O -Lnative=build/x86_64-unknown-linux-gnu/native/rust-test-helpers -Z unstable-options --unpretty=mir
$ echo $?
0
$ build/x86_64-unknown-linux-gnu/stage2/bin/rustc src/test/compile-fail/issue-37665.rs \
-L build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json \
-L build/x86_64-unknown-linux-gnu/test/compile-fail/issue-37665.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux \
-C prefer-dynamic -o build/x86_64-unknown-linux-gnu/test/compile-fail/issue-37665.stage2-x86_64-unknown-linux-gnu \
-Crpath -O -Lnative=build/x86_64-unknown-linux-gnu/native/rust-test-helpers -Z unstable-options
{"message":"compilation successful","code":null,"level":"error","spans":[{"file_name":"src/test/compile-fail/issue-37665.rs","byte_start":594,"byte_end":743,"line_start":18,"line_end":22,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn main() { //~ ERROR compilation successful","highlight_start":1,"highlight_end":45},{"text":"    let mut foo : String = \"hello\".to_string();","highlight_start":1,"highlight_end":48},{"text":"    foo.push(MAIN_SEPARATOR);","highlight_start":1,"highlight_end":30},{"text":"    println!(\"{}\", foo);","highlight_start":1,"highlight_end":25},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}
$ echo $?
101
