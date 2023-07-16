\n"},"level":"error","spans":[{"file_name":"/checkout/src/libcore/hash/mod.rs","byte_start":4730,"byte_end":4731,"line_start":185,"line_end":185,"column_start":13,"column_end":14,"is_primary":false,"text":[{"text":"    fn hash<H: Hasher>(&self, state: &mut H);","highlight_start":13,"highlight_end":14}],"label":"declaration in trait here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":967,"byte_end":978,"line_start":38,"line_end":38,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    fn hash(&self, hasher: &mut impl Hasher) {}","highlight_start":33,"highlight_end":44}],"label":"expected generic parameter, found `impl Trait`","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0643]: method `hash` has incompatible signature for trait\n  --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:38:33\n   |\nLL |     fn hash(&self, hasher: &mut impl Hasher) {}\n   |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`\n   | \n  ::: /checkout/src/libcore/hash/mod.rs:185:13\n   |\nLL |     fn hash<H: Hasher>(&self, state: &mut H);\n   |             - declaration in trait here\n\n"}
[00:44:16] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendere-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:16] 
[00:44:16] 
[00:44:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:16] Build completed unsuccessfully in 0:02:14
[00:44:16] Build completed unsuccessfully in 0:02:14
[00:44:16] make: *** [check] Error 1
[00:44:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0480a4ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
