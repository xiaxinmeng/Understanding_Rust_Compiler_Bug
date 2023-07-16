plain
Resolving deltas: 100% (613503/613503), completed with 4872 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:43:58] ..............................................................................i.....................
[00:44:04] .....................i..............................................................................
---
[00:44:39] ..................................................F.................................................
[00:44:46] i.........................................................................i.........................
[00:44:52] ....................................................................................................
[00:45:00] ....................................................................................................
[00:45:08] ....................................................................................................
known-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-invalid-fragment-spec.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-invalid-fragment-spec.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:45:09] {"message":"invalid fragment specifier `foo`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macro-invalid-fragment-spec.rs","byte_start":490,"byte_end":496,"line_start":12,"line_end":12,"column_start":6,"column_end":12,"is_primary":true,"text":[{"text":"    ($x:foo) => ()","highlight_start":6,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `literal`, `path`, `meta`, `tt`, `item` and `vis`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: invalid fragment specifier `foo`\n  --> /checkout/src/test/ui/macro-invalid-fragment-spec.rs:12:6\n   |\nLL |     ($x:foo) => ()\n   |      ^^^^^^\n   |\n   = help: valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `literal`, `path`, `meta`, `tt` "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:09] expected success, got: exit code: 101
[00:45:09]
[00:45:09]
[00:45:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:09] Build completed unsuccessfully in 0:02:28
[00:45:09] make: *** [check] Error 1
[00:45:09] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
