plain
[00:47:21] test [ui] ui/nll/issue-47022.rs ... ok
[00:47:21] test [ui] ui/nll/issue-47470.rs ... ok
[00:47:21] test [ui] ui/nll/issue-48238.rs ... ok
[00:47:21] test [ui] ui/nll/issue-43058.rs ... ok
[00:47:21] test [ui] ui/nll/issue-48070.rs#lxl ... ok
[00:47:21] test [ui] ui/nll/issue-48070.rs#nll ... ok
[00:47:21] test [ui] ui/nll/maybe-initialized-drop-with-fragment.rs ... ok
[00:47:21] test [ui] ui/nll/maybe-initialized-drop-with-uninitialized-fragments.rs ... ok
[00:47:21] test [ui] ui/nll/maybe-initialized-drop.rs ... ok
[00:47:21] test [ui] ui/nll/maybe-initialized-drop-uninitialized.rs ... ok
---
[00:48:06] test [ui (nll)] ui/nll/issue-47388.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/issue-16223.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/issue-47470.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/issue-48238.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/issue-48070.rs#lxl ... ok
[00:48:06] test [ui (nll)] ui/nll/maybe-initialized-drop-implicit-fragment-drop.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/maybe-initialized-drop-implicit-fragment-drop.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/issue-48070.rs#nll ... ok
[00:48:06] test [ui (nll)] ui/nll/maybe-initialized-drop-uninitialized.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/maybe-initialized-drop-with-uninitialized-fragments.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/maybe-initialized-drop.rs ... ok
[00:48:06] test [ui (nll)] ui/nll/return-ref-mut-issue-46557.rs ... ok
---
[00:50:19] test [ui] ui/nll/issue-48238.rs ... ok
[00:50:19] test [ui] ui/nll/issue-43058.rs ... ok
[00:50:19] test [ui] ui/nll/maybe-initialized-drop-implicit-fragment-drop.rs ... ok
[00:50:20] test [ui] ui/nll/maybe-initialized-drop-with-fragment.rs ... ok
[00:50:20] test [ui] ui/nll/issue-48070.rs#lxl ... ok
[00:50:20] test [ui] ui/nll/maybe-initialized-drop-with-uninitialized-fragments.rs ... ok
[00:50:20] test [ui] ui/nll/issue-48070.rs#nll ... ok
[00:50:20] test [ui] ui/nll/return-ref-mut-issue-46557.rs ... ok
[00:50:20] test [ui] ui/nll/trait-associated-constant.rs ... ok
[00:50:20] test [ui] ui/nll/maybe-initialized-drop-uninitialized.rs ... ok
[00:50:20] test [ui] ui/nll/ty-outlives/impl-trait-captures.rs ... ok
---
[00:50:37] 
[00:50:37] ---- [ui] ui/changing-crates.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/changing-crates.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'changing-crates.rs'
[00:50:37] 
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/changing-crates.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/changing-crates.rs","byte_start":745,"byte_end":760,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/changing-crates.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/changing-crates/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/changing-crates.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-change-significant-cfg.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/svh-change-significant-cfg.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-change-significant-cfg.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-change-significant-cfg.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-change-significant-cfg.rs","byte_start":735,"byte_end":750,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/svh-change-significant-cfg.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-significant-cfg/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-change-significant-cfg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-change-lit.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/svh-change-lit.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-change-lit.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-change-lit.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-change-lit.rs","byte_start":723,"byte_end":738,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/svh-change-lit.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-lit/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-change-lit.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-change-trait-bound.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/svh-change-trait-bound.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-change-trait-bound.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-change-trait-bound.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-change-trait-bound.rs","byte_start":731,"byte_end":746,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/svh-change-trait-bound.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-trait-bound/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-change-trait-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-change-type-arg.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/svh-change-type-arg.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-change-type-arg.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-change-type-arg.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-change-type-arg.rs","byte_start":728,"byte_end":743,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/svh-change-type-arg.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-arg/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-change-type-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-change-type-static.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/svh-change-type-static.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-change-type-static.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-change-type-static.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-change-type-static.rs","byte_start":731,"byte_end":746,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/svh-change-type-static.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-static/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-change-type-static.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-change-type-ret.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `a`: $PATH_a
[00:50:37] 10            crate `b`: $PATH_b
[00:50:37] +            crate `b`: $PATH_b
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/svh-change-type-ret.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-change-type-ret.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-change-type-ret.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `a` which `b` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-change-type-ret.rs","byte_start":728,"byte_end":743,"line_start":20,"line_end":20,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary/liba.rlib\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary/libb.so\ncrate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary/libb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `a` which `b` depends on\n  --> /checkout/src/test/ui/svh-change-type-ret.rs:20:1\n   |\nLL | extern crate b; //~ ERROR: found possibly newer version of crate `a` which `b` depends on\n   | ^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary/liba.rlib\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary/libb.so\n           crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-change-type-ret/auxiliary/libb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-change-type-ret.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
[00:50:37] 
[00:50:37] ---- [ui] ui/svh-use-trait.rs stdout ----
[00:50:37]  diff of stderr:
[00:50:37] 
[00:50:37] 8    = note: the following crate versions were found:
[00:50:37] 9            crate `uta`: $PATH_uta
[00:50:37] 10            crate `utb`: $PATH_utb
[00:50:37] +            crate `utb`: $PATH_utb
[00:50:37] 12 error: aborting due to previous error
[00:50:37] 13 
[00:50:37] 
[00:50:37] 
[00:50:37] 
[00:50:37] The actual stderr differed from the expected stderr.
[00:50:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/svh-use-trait.stderr
[00:50:37] To update references, run this command from build directory:
[00:50:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'svh-use-trait.rs'
[00:50:37] error: 1 errors occurred comparing output.
[00:50:37] status: exit code: 101
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/svh-use-trait.rs" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary" "-A" "unused"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] {"message":"found possibly newer version of crate `uta` which `utb` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/svh-use-trait.rs","byte_start":948,"byte_end":965,"line_start":25,"line_end":25,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"extern crate utb; //~ ERROR: found possibly newer version of crate `uta` which `utb` depends","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `uta`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary/libuta.rlib\ncrate `utb`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary/libutb.so\ncrate `utb`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary/libutb.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `uta` which `utb` depends on\n  --> /checkout/src/test/ui/svh-use-trait.rs:25:1\n   |\nLL | extern crate utb; //~ ERROR: found possibly newer version of crate `uta` which `utb` depends\n   | ^^^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `uta`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary/libuta.rlib\n           crate `utb`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary/libutb.so\n           crate `utb`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/svh-use-trait/auxiliary/libutb.rlib\n\n"}
[00:50:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:37] {"message":"For more information about this error, try `rustc --explain E0460`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0460`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/svh-use-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3013:9
[00:50:37] 
---
[00:50:37] 
[00:50:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:50:37] 
[00:50:37] 
[00:50:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "ui" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:37] 
[00:50:37] 
[00:50:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:50:37] Build completed unsuccessfully in 0:47:31
