plain
[00:44:12] ....................................................................................................
[00:44:16] ....................................................................................................
[00:44:19] ....................................................................................................
[00:44:23] ....................................................................................................
[00:44:27] ............................................F.......................................................
[00:44:30] .......F............................................................................................
[00:44:37] .i.........................................................................................i........
[00:44:40] ....................................................................................................
[00:44:44] ....................................................................................................
[00:44:47] ....................................................................................................
[00:44:47] ....................................................................................................
[00:44:51] .........................................i..........................................................
[00:44:53] ............................
[00:44:53] failures:
[00:44:53] 
[00:44:53] ---- [ui] ui/issue-46209-private-enum-variant-reexport.rs stdout ----
[00:44:53] diff of stderr:
[00:44:53] 
[00:44:53] 1 error: enum is private and its variants cannot be re-exported
[00:44:53] -    |
[00:44:53] -    |
[00:44:53] - LL |     pub use self::Professor::*;
[00:44:53] - ...
[00:44:53] - ...
[00:44:53] - LL |     enum Professor {
[00:44:53] -    |     -------------- help: consider making the enum public: `pub enum Professor`
[00:44:53] - 
[00:44:53] - error: enum is private and its variants cannot be re-exported
[00:44:53] -    |
[00:44:53] -    |
[00:44:53] - LL |     pub use self::Crewman::*;
[00:44:53] - ...
[00:44:53] - ...
[00:44:53] - LL |     crate enum Crewman {
[00:44:53] -    |     ------------------ help: consider making the enum public: `pub enum Crewman`
[00:44:53] - 
[00:44:53] - error: enum is private and its variants cannot be re-exported
[00:44:53] 21    |
[00:44:53] 21    |
[00:44:53] 22 LL |     pub use self::PettyOfficer::*;
[00:44:53] 
[00:44:53] 33 ... Lieutenant","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variant `JuniorGrade` is private and cannot be re-exported\n  --> /checkout/src/test/ui/issue-46209-private-enum-variant-reexport.rs:16:32\n   |\nLL |     pub use self::Lieutenant::{JuniorGrade, Full};\n   |                                ^^^^^^^^^^^\n...\nLL |     enum Lieutenant {\n   |     --------------- help: consider making the enum public: `pub enum Lieutenant`\n\n"}
[00:44:53] {"message":"enum is private and its variants cannot be re-exported","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-46209-private-enum-variant-reexport.rs","byte_start":929,"byte_end":945,"line_start":21,"line_end":21,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    pub use self::Crewman::*;","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider making the enum public","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issue-46209-private-enum-variant-reexport.rs","byte_start":1302,"byte_end":1320,"line_start":43,"line_end":43,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    crate enum Crewman {","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"pub enum Crewman","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: enum is private and its variants cannot be re-exported\n  --> /checkout/src/test/ui/issue-46209-private-enum-varian: Fn()>(&self, f: &F) {
[00:44:53] 
[00:44:53] The actual stderr differed from the expected stderr.
[00:44:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22638/issue-22638.stderr
[00:44:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22638/issue-22638.stderr
[00:44:53] To update references, rerun the tests and pass the `--bless` flag
[00:44:53] To only update this specific test, also pass `--test-args issue-22638.rs`
[00:44:53] error: 1 errors occurred comparing output.
[00:44:53] status: exit code: 101
[00:44:53] status: exit code: 101
[00:44:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-22638.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22638/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-22638/auxiliary" "-A" "unused"
[00:44:53] ------------------------------------------
[00:44:53] 
[00:44:53] ------------------------------------------
[00:44:53] stderr:
[00:44:53] stderr:
[00:44:53] ------------------------------------------
[00:44:53] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:44:53] {"message":"reached the type-length limit while instantiating `D::matches::<[closure@/checkout/src/test/ui/issue-22638.rs:50:23...`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-22638.rs","byte_start":1261,"byte_end":1449,"line_start":60,"line_end":64,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn matches<F: Fn()>(&self, f: &F) {","highlight_start":5,"highlight_end":44},{"text":"        //~^ ERROR reached the type-length limit while instantiating `D::matches::<[closure","highlight_start":1,"highlight_end":92},{"text":"        let &D(ref a) = self;","highlight_start":1,"highlight_end":30},{"text":"        a.matches(f)","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider adding a `#![type_length_limit=\"40000000\"]` attribute to your crate","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: reached the type-length limit while instantiating `D::matches::<[closure@/checkout/src/test/ui/issue-22638.rs:50:23...`\n  --> /checkout/src/test/ui/issue-22638.rs:60:5\n   |\nLL | /     pub fn matches<F: Fn()>(&self, f: &F) {\nLL | |         //~^ ERROR reached the type-length limit while instantiating `D::matches::<[closure\nLL | |         let &D(ref a) = self;\nLL | |         a.matches(f)\nLL | |     }\n   | |_____^\n   |\n   = note: consider adding a `#![type_length_limit=\"40000000\"]` attribute to your crate\n\n"}
[00:44:53] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:53] ------------------------------------------
[00:44:53] 
[00:44:53] thread '[ui] ui/issue-22638.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:44:53] 
---
[00:44:53] test result: FAILED. 2120 passed; 2 failed; 6 ignored; 0 measured; 0 filtered out
[00:44:53] 
[00:44:53] 
[00:44:53] 
[00:44:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:53] 
[00:44:53] 
[00:44:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstranknown-linux-gnu/release
122468 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
