plain
    100% |████████████████████████████████| 4.2MB 298kB/s 
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a0/70/2c27740f08e477499ce19eefe05dbcae6f19fdc49e9e82ce4768be0643b9/pyasn1-0.4.3-py2.py3-none-any.whl (72kB)
    14% |████▌                           | 10kB 42.9MB/s eta 0:00:01
    28% |█████████                       | 20kB 43.1MB/s eta 0:00:01
    42% |█████████████▌                  | 30kB 50.4MB/s eta 0:00:01
    56% |██████████████████              | 40kB 54.5MB/s eta 0:00:01
---
[00:47:39] ....................................................................................................
[00:47:44] ....................................................................................................
[00:47:50] ...........................................................................i........................
[00:47:55] ....................................................i...............................................
[00:48:00] ........................................................................ii..................F.......
[00:48:06] ......F.............................................................................................
[00:48:12] ...................................................................................i................
[00:48:15] .iiiiiiiii...................................................
[00:48:15] 
[00:48:15] ---- [ui] ui/rfc-2166-underscore-imports/basic.rs stdout ----
[00:48:15] diff of stderr:
[00:48:15] 
[00:48:15] 
[00:48:15] 20   --> $DIR/basic.rs:33:5
[00:48:15] 21    |
[00:48:15] 22 LL |     extern crate core as _; //~ WARN unused extern crate
[00:48:15] +    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: remove it
[00:48:15] 24    |
[00:48:15] 25 note: lint level defined here
[00:48:15] 26   --> $DIR/basic.rs:14:25
[00:48:15] 26   --> $DIR/basic.rs:14:25
[00:48:15] 
[00:48:15] 
[00:48:15] The actual stderr differed from the expected stderr.
[00:48:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2166-underscore-imports/basic/basic.stderr
[00:48:15] To update references, rerun the tests and pass the `--bless` flag
[00:48:15] To only update this specific test, also pass `--test-args rfc-2166-underscore-imports/basic.rs`
[00:48:15] error: 1 errors occurred comparing output.
[00:48:15] status: exit code: 0
[00:48:15] status: exit code: 0
[00:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2166-underscore-imports/basic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2166-underscore-imports/basic/auxiliary" "-A" "unused"
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] stderr:
[00:48:15] stderr:
[00:48:15] ------------------------------------------
[00:48:15] {"message":"unused import: `m::Tr1 as _`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs","byte_start":782,"byte_end":793,"line_start":31,"line_end":31,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    use m::Tr1 as _; //~ WARN unused import","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs","byte_start":524,"byte_end":538,"line_start":14,"line_end":14,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"#![warn(unused_imports, unused_extern_crates)]","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `m::Tr1 as _`\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs:31:9\n   |\nLL |     use m::Tr1 as _; //~ WARN unused import\n   |         ^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs:14:9\n   |\nLL | #![warn(unused_imports, unused_extern_crates)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:48:15] {"message":"unused import: `S as _`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs","byte_start":826,"byte_end":832,"line_start":32,"line_end":32,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    use S as _; //~ WARN unused import","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `S as _`\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs:32:9\n   |\nLL |     use S as _; //~ WARN unused import\n   |         ^^^^^^\n\n"}
[00:48:15] {"message":"unused extern crate","code":{"code":"unused_extern_crates","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs","byte_start":861,"byte_end":884,"line_start":33,"line_end":33,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    extern crate core as _; //~ WARN unused extern crate","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs","byte_start":540,"byte_end":560,"line_start":14,"line_end":14,"column_start":25,"column_end":45,"is_primary":true,"text":[{"text":"#![warn(unused_imports, unused_extern_crates)]","highlight_start":25,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs","byte_start":861,"byte_end":884,"line_start":33,"line_end":33,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    extern crate core as _; //~ WARN unused extern crate","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused extern crate\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs:33:5\n   |\nLL |     extern crate core as _; //~ WARN unused extern crate\n   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: remove it\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs:14:25\n   |\nLL | #![warn(unused_imports, unused_extern_crates)]\n   |                         ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] thread '[ui] ui/rfc-2166-underscore-imports/basic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] 
[00:48:15] ---- [ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs stdout ----
[00:48:15] diff of fixed:
[00:48:15] 
[00:48:15] 26 extern crate edition_lint_paths as bar;
[00:48:15] 27 
[00:48:15] 28 fn main() {
[00:48:15] +     // This is not considered to *use* the `extern crate` in Rust 2018:
[00:48:15] 29     use edition_lint_paths::foo;
[00:48:15] 30     foo();
[00:48:15] + 
[00:48:15] +     // But this should be a use of the (renamed) crate:
[00:48:15] 31     crate::bar::foo();
[00:48:15] 32 }
[00:48:15] 
[00:48:15] 
[00:48:15] The actual fixed differed from the expected fixed.
[00:48:15] The actual fixed differed from the expected fixed.
[00:48:15] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/extern-crate-idiomatic-in-2018.fixed
[00:48:15] To update references, rerun the tests and pass the `--bless` flag
[00:48:15] To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic-in-2018.rs`
[00:48:15] error: 1 errors occurred comparing output.
[00:48:15] status: exit code: 101
[00:48:15] status: exit code: 101
[00:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/auxiliary" "-A" "unused"
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] stderr:
[00:48:15] stderr:
[00:48:15] ------------------------------------------
[00:48:15] {"message":"unused extern crate","code":{"code":"unused_extern_crates","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":771,"byte_end":803,"line_start":23,"line_end":23,"column_start":1,"column_end":33,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths;","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":726,"byte_end":746,"line_start":20,"line_end":20,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![deny(unused_extern_crates)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs","byte_start":771,"byte_end":803,"line_start":23,"line_end":23,"column_start":1,"column_end":33,"is_primary":true,"text":[{"text":"extern crate edition_lint_paths;","highlight_start":1,"highlight_end":33}],"label":null,"suggested_replacement":"","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused extern crate\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:23:1\n   |\nLL | extern crate edition_lint_paths;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove it\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:20:9\n   |\nLL | #![deny(unused_extern_crates)]\n   |         ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:48:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] 
[00:48:15] thread '[ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[00:48:15] 
[00:48:15] failures:
[00:48:15]     [ui] ui/rfc-2166-underscore-imports/basic.rs
[00:48:15]     [ui] ui/rfc-2166-underscore-imports/basic.rs
[00:48:15]     [ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs
[00:48:15] test result: FAILED. 1443 passed; 2 failed; 16 ignored; 0 measured; 0 filtered out
[00:48:15] 
[00:48:15] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
