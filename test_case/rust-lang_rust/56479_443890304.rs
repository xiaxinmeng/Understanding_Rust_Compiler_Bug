plain
travis_time:end:05109c63:start=1543872225285282425,finish=1543872226455712808,duration=1170430383
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:41] .................................................................................................... 3000/5105
[00:47:44] ............................................................................i....................... 3100/5105
[00:47:47] .................................................................................................... 3200/5105
[00:47:50] .......................................ii..i..ii.................................................... 3300/5105
[00:47:54] ...........F...FF...FFF...FFF.............................F....F........F......F...........F........ 3400/5105
[00:47:57] ......FFFF............................FF..F..........F........FFFFFF...F..................FFFF...... 3500/5105
[00:48:02] ......................................i............................................................. 3700/5105
[00:48:03] ..............................................................................................i..... 3800/5105
[00:48:04] .................................................................................................... 3900/5105
[00:48:11] .................................................................................................... 4000/5105
[00:48:11] .................................................................................................... 4000/5105
[00:48:14] .................................................................................................... 4100/5105
[00:48:17] ........................................................................F........................... 4200/5105
[00:48:26] .................................................................................................... 4400/5105
[00:48:30] .................................................................................................... 4500/5105
[00:48:33] .................................................................................................... 4600/5105
[00:48:36] ....................................................................i............................... 4700/5105
---
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/escape-argument-callee.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 9                for<'r, 's, 't0> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) i32))
[00:48:48] 11 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 13   --> $DIR/escape-argument-callee.rs:36:45
[00:48:48] 13   --> $DIR/escape-argument-callee.rs:36:45
[00:48:48] 14    |
[00:48:48] 15 LL |         let mut closure = expect_sig(|p, y| *p = y);
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/escape-argument-callee.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/closure-requirements/escape-argument-callee.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/escape-argument-callee/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs","byte_start":1353,"byte_end":1366,"line_start":36,"line_end":36,"column_start":38,"column_end":51,"is_primary":true,"text":[{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":38,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:9 ~ escape_argument_callee[317d]::test[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) mut &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) i32, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) i32))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure,"byte_start":1360,"byte_end":1366,"line_start":36,"line_end":36,"column_start":45,"column_end":51,"is_primary":true,"text":[{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":45,"highlight_end":51}],"label":"assignment requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:36:45\n   |\nLL |         let mut closure = expect_sig(|p, y| *p = y);\n   |                                       -  -  ^^^^^^ assignment requires that `'1` must outlive `'2`\n   |                                       |  |\n   |                                       |  has type `&'1 i32`\n   |                                       has type `&mut &'2 i32`\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs","byte_start":1241,"byte_end":1439,"line_start":30,"line_end":42,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn test() {","highlight_start":1,"highlight_end":12},{"text":"    let x = 44;","highlight_start":1,"highlight_end":16},{"text":"    let mut p = &x;","highlight_start":1,"highlight_end":20},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    {","highlight_start":1,"highlight_end":6},{"text":"        let y = 22;","highlight_start":1,"highlight_end":20},{"text":"        let mut closure = expect_sig(|p, y| *p = y);","highlight_start":1,"highlight_end":53},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"        closure(&mut p, &y);","highlight_start":1,"highlight_end":29},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    deref(p);","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:3 ~ escape_argument_callee[317d]::test[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/escape-argument-callee.rs:30:1\n   |\nLL | / fn test() {\nLL | |     let x = 44;\nLL | |     let mut p = &x;\nLL | |\n...  |\nLL | |     deref(p);\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:3 ~ escape_argument_callee[317d]::test[0]) with substs []\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/escape-argument-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 16    = note: late-bound region is '_#5r
[00:48:48] 17    = note: late-bound region is '_#6r
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 20   --> $DIR/propagate-approximated-fail-no-postdom.rs:56:13
[00:48:48] 21    |
[00:48:48] 21    |
[00:48:48] 22 LL |         |_outlives1, _outlives2, _outlives3, x, y| {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-fail-no-postdom.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fai: late-bound region is '_#6r\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs","byte_start":1570,"byte_end":1580,"line_start":53,"line_end":53,"column_start":10,"column_end":20,"is_primary":false,"text":[{"text":"        |_outlives1, _outlives2, _outlives3, x, y| {","highlight_start":10,"highlight_end":20}],"label":"has type `std::cell::Cell<&&'1 u32>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs","byte_start":1594,"byte_end":1604,"line_start":53,"line_end":53,"column_start":34,"column_end":44,"is_primary":false,"text":[{"text":"        |_outlives1, _outlives2, _outlives3, x, y| {","highlight_start":34,"highlight_end":44}],"label":"has type `std::cell::Cell<&'2 &u32>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs","byte_start":1692,"byte_end":1709,"line_start":56,"line_end":56,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"            demand_y(x, y, p) //~ ERROR","highlight_start":13,"highlight_end":30}],"label":"argument requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_approximated_fail_no_postdom[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:48:1\n   |\nLL | / fn supply<'a, 'b, 'c>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>, cell_c: Cell<&'c u32>) {\nLL | |     establish_relationships(\nLL | |         cell_a,\nLL | |         cell_b,\n...  |\nLL | |     );\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_approximated_fail_no_postdom[317d]::supply[0]) with substs []\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/propagate-approximated-ref.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 33    |
[00:48:48] 34    = note: defining type: DefId(0/0:6 ~ propagate_approximated_ref[317d]::supply[0]) with substs []
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 37   --> $DIR/propagate-approximated-ref.rs:56:9
[00:48:48] 38    |
[00:48:48] 38    |
[00:48:48] 39 LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-ref.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"External requirements","code":null,"leveldex(0:0), 't1)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#4r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:53:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |  _______________________________________________^\nLL | |\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ ERROR unsatisfied lifetime constraints\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_approximated_ref[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n           ]\n   = note: late-bound region is '_#3r\n   = note: late-bound region is '_#4r\n   = note: number of external vids: 5\n   = note: where '_#1r: '_#2r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs","byte_start":1749,"byte_end":2021,"line_start":52,"line_end":59,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":1,"highlight_end":79},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ ERROR unsatisfied lifetime constraints","highlight_start":1,"highlight_end":52},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_approximated_ref[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:52:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |\nLL | |         // Only works if 'x: 'y:\n...  |\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_approximated_ref[317d]::supply[0]) with substs []\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs","byte_start":1759,"byte_end":1761,"line_start":52,"line_end":52,"column_start":11,"column_end":13,"is_primary":false,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":11,"highlight_end":13}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs","byte_start":1763,"byte_end":1765,"line_start":52,"line_end":52,"column_start":15,"column_end":17,"is_primary":false,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":15,"highlight_end":17}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs","byte_start":1936,"byte_end":1959,"line_start":56,"line_end":56,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":"argument requires that `'a` must outlive `'b`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:56:9\n   |\nLL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\n   |           --  -- lifetime `'b` defined here\n   |           |\n   |           lifetime `'a` defined here\n...\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/propagate-approximated-ref.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/propagate-approximated-val.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 33    |
[00:48:48] 34    = note: defining type: DefId(0/0:6 ~ propagate_approximated_val[317d]::test[0]) with substs []
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 37   --> $DIR/propagate-approximated-val.rs:49:9
[00:48:48] 38    |
[00:48:48] 38    |
[00:48:48] 39 LL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-val.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propull,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:46:45\n   |\nLL |       establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {\n   |  _____________________________________________^\nLL | |\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(outlives1, outlives2, x.get())\nLL | |         //~^ ERROR unsatisfied lifetime constraints\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_approximated_val[317d]::test[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's> extern \"rust-call\" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>))\n           ]\n   = note: late-bound region is '_#3r\n   = note: late-bound region is '_#4r\n   = note: number of external vids: 5\n   = note: where '_#1r: '_#2r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs","byte_start":1318,"byte_end":1600,"line_start":45,"line_end":52,"column_start":1,"column_end":2,"is_primarted-val.rs","byte_start":1326,"byte_end":1328,"line_start":45,"line_end":45,"column_start":9,"column_end":11,"is_primary":false,"text":[{"text":"fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":9,"highlight_end":11}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs","byte_start":1330,"byte_end":1332,"line_start":45,"line_end":45,"column_start":13,"column_end":15,"is_primary":false,"text":[{"text":"fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":13,"highlight_end":15}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs","byte_start":1499,"byte_end":1538,"line_start":49,"line_end":49,"column_start":9,"column_end":48,"is_primary":true,"text":[{"text":"        demand_y(outlives1, outlives2, x.get())","highlight_start":9,"highlight_end":48}],"label":"argument requires that `'a` must outlive `'b`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:49:9\n   |\nLL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\n   |         --  -- lifetime `'b` defined here\n   |         |\n   |         lifetime `'a` defined here\n...\nLL |         demand_y(outlives1, outlives2, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/propagate-approximated-val.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 16    = note: late-bound region is '_#2r
[00:48:48] 17    = note: late-bound region is '_#3r
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 20   --> $DIR/propagate-fail-to-approximate-longer-no-bounds.rs:47:9
[00:48:48] 21    |
[00:48:48] 21    |
[00:48:48] 22 LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1575,"byte_end":1684,"line_start":45,"line_end":49,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":47,"highlight_end":66},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:45:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\n   |  _______________________________________________^\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ ERROR\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: Defsupply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":1,"highlight_end":66},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:44:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ ERROR\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]) with substs []\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] 
[00:48:48] ---------------------target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1751,"byte_end":1873,"line_start":49,"line_end":53,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#4r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:49:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |  _______________________________________________^\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ ERROR\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supp,"column_start":60,"column_end":70,"is_primary":false,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":60,"highlight_end":70}],"label":"has type `&std::cell::Cell<&'2 &u32>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1825,"byte_end":1848,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":"argument requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:51:9\n   |\nLL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |                                                ----------  ---------- has type `&std::cell::Cell<&'2 &u32>`\n   |                                                |\n   |                                                has type `&std::cell::Cell<&'1 &u32>`\nLL |         // Only works if 'x: 'y:\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1639,"byte_end":1877,"line_start":48,"line_end":54,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":1,"highlight_end":79},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:48:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ ERROR\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]) with substs []\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/region-lbr-named-does-not-outlive-static.rs:19:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a>(x: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/region-lbr-named-does-not-outlive-static.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/region-lbr-named-does-not-outlive-static.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs","byte_start":779,"byte_end":781,"line_start":18,"line_end":18,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo<'a>(x: &'a u32) -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs","byte_start":817,"byte_end":820,"line_start":19,"line_end":19,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    &*x","highlight_start":5,"highlight_end":8}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs:19:5\n   |\nLL | fn foo<'a>(x: &'a u32) -> &'static u32 {\n   |        -- lifetime `'a` defined here\nLL |     &*x\n   |     ^^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/region-lbr1-does-not-outlive-ebr2.rs:19:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/region-lbr1-does-not-outlive-ebr2.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this sp"/checkout/src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs","byte_start":783,"byte_end":785,"line_start":18,"line_end":18,"column_start":12,"column_end":14,"is_primary":false,"text":[{"text":"fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {","highlight_start":12,"highlight_end":14}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs","byte_start":828,"byte_end":831,"line_start":19,"line_end":19,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    &*x","highlight_start":5,"highlight_end":8}],"label":"function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs:19:5\n   |\nLL | fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {\n   |        --  -- lifetime `'b` defined here\n   |        |\n   |        lifetime `'a` defined here\nLL |     &*x\n   |     ^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":736,"byte_end":744,"line_start":21,"line_end":21,"column_start":16,"column_end":24,"is_primary":true,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":16,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:9 ~ return_wrong_bound_region[317d]::test[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) i32)) -> &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:21:16\n   |\nLL |     expect_sig(|a, b| b); // ought to return `a`\n   |                ^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:9 ~ return_wrong_bound_region[317d]::test[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) i32)) -> &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32\n           ]\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":740,"byte_end":741,"line_start":21,"line_end":21,"column_start":20,"column_end":21,"is_primary":false,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":20,"highlight_end":21}],"label":"has type `&'1 i32`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":737,"byte_end":738,"line_start":21,"line_end":21,"column_start":17,"column_end":18,"is_primary":false,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":17,"highlight_end":18}],"label":"has type `&'2 i32`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-r0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:20:1\n   |\nLL | / fn test() {\nLL | |     expect_sig(|a, b| b); // ought to return `a`\nLL | |     //~^ ERROR\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:3 ~ return_wrong_bound_region[317d]::test[0]) with substs []\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/closure-requirements/return-wrong-bound-region.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/issue-48238.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/issue-48238.rs:21:13
[00:48:48] 3    |
[00:48:48] 4 LL |     move || use_val(&orig); //~ ERROR
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/issue-48238.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/issue-48238.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/issue-48238.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-48238.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-48238.rs","byte_start":611,"byte_end":618,"line_start":21,"line_end":21,"column_start":5,"column_end":12,"is_primary":false,"text":[{"text":"    move || use_val(&orig); //~ ERROR","highlight_start":5,"highlight_end":12}],"label":"lifetime `'1` represents this closure's body","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-48238.rs","byte_start":617,"byte_end":618,"line_start":21,"line_end":21,"column_start":11,"column_end":12,"is_primary":false,"text":[{"text":"    move || use_val(&orig); //~ ERROR","highlight_start":11,"highlight_end":12}],"label":"return type of closure is &'2 u8","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-48238.rs","byte_start":619,"byte_end":633,"line_start":21,"line_end":21,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    move || use_val(&orig); //~ ERROR","highlight_start":13,"highlight_end":27}],"label":"returning this value requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"closure implements `Fn`, so references to captured variables can't escape the closure","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/issue-48238.rs:21:13\n   |\nLL |     move || use_val(&orig); //~ ERROR\n   |     ------- ^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`\n   |     |     |\n   |     |     return type of closure is &'2 u8\n   |     lifetime `'1` represents this closure's body\n   |\n   = note: closure implements `Fn`, so references to captured variables can't escape the closure\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/issue-48238.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/issue-50716.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/issue-50716.rs:25:14
[00:48:48] 3    |
[00:48:48] 4 LL | fn foo<'a, T: 'static>(s: Box<<&'a T as A>::X>)
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/issue-50716.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/issue-50716.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/issue-50716.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-50716.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-50716.rs","byte_start":635,"byte_end":637,"line_start":20,"line_end":20,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo<'a, T: 'static>(s: Box<<&'a T as A>::X>)","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-50716.rs","byte_start":751,"byte_end":753,"line_start":25,"line_end":25,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"    let _x = *s; //~ ERROR","highlight_start":14,"highlight_end":16}],"label":"proving this value is `Sized` requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/issue-50716.rs:25:14\n   |\nLL | fn foo<'a, T: 'static>(s: Box<<&'a T as A>::X>)\n   |        -- lifetime `'a` defined here\n...\nLL |     let _x = *s; //~ ERROR\n   |              ^^ proving this value is `Sized` requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/issue-50716.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/issue-52113.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/xt":[{"text":"fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {","highlight_start":16,"highlight_end":18}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-52113.rs","byte_start":1074,"byte_end":1076,"line_start":42,"line_end":42,"column_start":20,"column_end":22,"is_primary":false,"text":[{"text":"fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {","highlight_start":20,"highlight_end":22}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-52113.rs","byte_start":1244,"byte_end":1245,"line_start":47,"line_end":47,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x   //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'b`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/issue-52113.rs:47:5\n   |\nLL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {\n   |                --  -- lifetime `'b` defined here\n   |                |\n   |                lifetime `'a` defined here\n...\nLL |     x   //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'b`\n\n"}
[00:48:48] {"messu/test/ui/nll/issue-52742/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-52742.rs","byte_start":660,"byte_end":662,"line_start":24,"line_end":24,"column_start":35,"column_end":37,"is_primary":false,"text":[{"text":"    fn take_bar(&mut self, b: Bar<'_>) {","highlight_start":35,"highlight_end":37}],"label":"let's call this `'1`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-52742.rs","byte_start":642,"byte_end":651,"line_start":24,"line_end":24,"column_start":17,"column_end":26,"is_primary":false,"text":[{"text":"    fn take_bar(&mut self, b: Bar<'_>) {","highlight_start":17,"highlight_end":26}],"label":"has type `&mut Foo<'_, '2>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-52742.rs","byte_start":675,"byte_end":687,"line_start":25,"line_end":25,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        self.y = b.z","highlight_start":9,"highlight_end":21}],"label":"assignment requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/issue-52742.rs:25:9\n   |\nLL |     fn take_bar(&mut self, b: Bar<'_>) {\n   |                 ---------         -- let's call this `'1`\n   |                 |\n   |                 has type `&mut Foo<'_, '2>`\nLL |         self.y = b.z\n   |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/issue-52742.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/issue-55394.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/issue-55394.rs:21:9
[00:48:48] 3    |
[00:48:48] 4 LL |     fn new(bar: &mut Bar) -> Self {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/issue-55394.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/issue-55394.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/issue-55394.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55394.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-55394.rs","byte_start":571,"byte_end":572,"line_start":20,"line_end":20,"column_start":17,"column_end":18,"is_primary":false,"text":[{"text":"    fn new(bar: &mut Bar) -> Self {","highlight_start":17,"highlight_end":18}],"label":"let's call the lifetime of this reference `'1`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-55394.rs","byte_start":584,"byte_end":588,"line_start":20,"line_end":20,"column_start":30,"column_end":34,"is_primary":false,"text":[{"text":"    fn new(bar: &mut Bar) -> Self {","highlight_start":30,"highlight_end":34}],"label":"return type is Foo<'2>","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-55394.rs","byte_start":599,"byte_end":610,"line_start":21,"line_end":21,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        Foo { bar }","highlight_start":9,"highlight_end":20}],"label":"returning this value requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/issue-55394.rs:21:9\n   |\nLL |     fn new(bar: &mut Bar) -> Self {\n   |                 -            ---- return type is Foo<'2>\n   |                 |\n   |                 let's call the lifetime of this reference `'1`\nLL |         Foo { bar }\n   |         ^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/issue-55394.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/mir_check_cast_reify.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/mir_check_cast_reify.rs:47:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn bar<'a>(x: &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_reify/mir_check_cast_reify.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_reify/mir_check_cast_reify.stderr
[00:48:48] To updat, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/mir_check_cast_closure.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_closure.rs","byte_start":535,"byte_end":537,"line_start":15,"line_end":15,"column_start":12,"column_end":14,"is_primary":false,"text":[{"text":"fn bar<'a, 'b>() -> fn(&'a u32, &'b u32) -> &'a u32 {","highlight_start":12,"highlight_end":14}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_closure.rs","byte_start":531,"byte_end":5338] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 3    |
[00:48:48] 3    |
[00:48:48] 4 LL | fn bar<'a>(input: &'a u32, f: fn(&'a u32) -> &'a u32) -> &'static u32 {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/mir_check_cast_unsafe_fn.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/mir_check_cast_unsafe_fn.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/mir_check_cast_unsafe_fn.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs","byte_start":530,"byte_end":532,"line_start":15,"line_end":15,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn bar<'a>(input: &'a u32, f: fn(&'a u32) -> &'a u32) -> &'static u32 {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs","byte_start":777,"byte_end":785,"line_start":19,"line_end":19,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    unsafe { g(input) }","highlight_start":14,"highlight_end":22}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs:19:14\n   |\nLL | fn bar<'a>(input: &'a u32, f: fn(&'a u32) -> &'a u32) -> &'static u32 {\n   |        -- lifetime `'a` defined here\n...\nLL |     unsafe { g(input) }\n   |              ^^^^^^^^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/mir_check_cast_unsafe_fn.rs' panicked at 'explicit panic', src/tools/compiletest:48] ------------------------------------------
[00:48:48] thread '[ui] ui/nll/mir_check_cast_unsafe_fn.rs' panicked at 'explicit panic', src/tools/compiletest:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_unsize.rs","byte_start":553,"byte_end":555,"line_start":17,"line_end":17,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn bar<'a>(x: &'a u32) -> &'static dyn Debug {","highlight_start":8,"highlight_end":10}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_unsize.rs","byte_start":597,"byte_end":598,"line_start":18,"line_end":18,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/mir_check_cast_unsize.rs:18:5\n   |\nLL | fn bar<'a>(x: &'a u32) -> &'static dyn Debug {\n   |        -- lifetime `'a` defined here\nLL |     x\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/mir_check_cast_unsize.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/ty-outlives/projection-one-region-closure.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] 40    |
[00:48:48] 41    = help: consider adding an explicit lifetime bound `T: ReFree(DefId(0/0:8 ~ projection_one_region_closure[317d]::no_relationships_late[0]), BrNamed(crate0:DefIndex(1:16), 'a))`...
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 44   --> $DIR/projection-one-region-closure.rs:55:39
[00:48:48] 45    |
[00:48:48] 45    |
[00:48:48] 46 LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
[00:48:48] 94    |
[00:48:48] 94    |
[00:48:48] 95    = help: consider adding an explicit lifetime bound `T: ReEarlyBound(0, 'a)`...
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 98   --> $DIR/projection-one-region-closure.rs:66:39
[00:48:48] 99    |
[00:48:48] 99    |
[00:48:48] 100 LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-closure.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1572,"byte_end":1598,"line_start":55,"line_end":55,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:19 ~ projection_one_region_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"late-bound region is '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where T: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:55:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:19 ~ projection_one_region_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n           ]\n   = note: late-bound region is '_#3r\n   = note: number of external vids: 4\n   = note: where T: '_#2r\n   = note: where '_#1r: '_#2r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1453,"byte_end":1680,"line_start":51,"line_end":58,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":62},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'b>,","highlight_start":1,"highlight_end":21},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"    //~^ ERROR the parameter type `T` may not live long enough","highlight_start":1,"highlight_end":63},{"text":"    //~| ERROR","highlight_start":1,"highlight_end":15},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:8 ~ projection_one_region_closure[317d]::no_relationships_late[0]) with substs [\n    '_#1r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:51:1\n   |\nLL | / fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'b>,\nLL | | {\n...  |\nLL | |     //~| ERROR\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:8 ~ projection_one_region_closure[317d]::no_relationships_late[0]) with substs [\n               '_#1r,\n               T\n           ]\n\n"}
[00:48:48] {"message":"the parameter type `T` may not live long enough","code":{"code":"E0309","explanation":"\nThe type definition contains some field whose type\nrequires an outlives annotation. Outlives annotations\n(e.g., `T: 'a`) are used to guarantee that all the data in T is valid\nfor at least the lifetime `'a`. This scenario most commonly\narises when the type contains an associated type reference\nlike `<T as SomeTrait<'a>>::Outui/nll/ty-outlives/projection-one-region-closure.rs:55:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider adding an explicit lifetime bound `T: ReFree(DefId(0/0:8 ~ projection_one_region_closure[317d]::no_relationships_late[0]), BrNamed(crate0:DefIndex(1:16), 'a))`...\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1482,"byte_end":1484,"line_start":51,"line_end":51,"column_start":30,"column_end":32,"is_primary":false,"text":[{"text":"fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":30,"highlight_end":32}],"label":"lifetime `'b` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1478,"byte_end":1480,"line_start":51,"line_end":51,"column_start":26,"column_end":28,"is_primary":false,"text":[{"text":"fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":26,"highlight_end":28}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1582,"byte_end":1598,"line_start":55,"line_end":55,"column_start":39,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highldren":[],"rendered":null},{"message":"where T: '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#2r: '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:66:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:23 ~ projection_one_region_closure[317d]::no_relationships_early[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               '_#2r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#3r ()>, T))\n           ]\n   = note: number of external vids: 4\n   = note: where T: '_#3r\n   = note: where '_#2r: '_#3r\n\n"}
[00:48:48] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1699,"byte_end":1939,"line_start":61,"line_end":69,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)","highlight_start":1,"highlight_end":63},{"text":"where","highlight_start":1,"highlight_end":6},{"text":"    T: Anything<'b>,","highlight_start":1,"highlight_end":21},{"text":"    'a: 'a,","highlight_start":1,"highlight_end":12},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":1,"highlight_end":57},{"text":"    //~^ ERROR the parameter type `T` may not live long enough","highlight_start":1,"highlight_end":63},{"text":"    //~| ERROR","highlight_start":1,"highlight_end":15},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:9 ~ projection_one_region_closure[317d]::no_relationships_early[0]) with substs [\n    '_#1r,\n    '_#2r,\n    T\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:61:1\n   |\nLL | / fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)\nLL | | where\nLL | |     T: Anything<'b>,\nLL | |     'a: 'a,\n...  |\nLL | |     //~| ERROR\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:9 ~ projection_one_region_closure[317d]::no_relationships_early[0]) with substs [\n               '_#1r,\n               '_#2r,\n               T\n           ]\n\n"}
[00:48:48] {"message":"the parameter type `T` may not live long enough","code":{"code":"E0309","explanation":"\nThe type definition contains some field whose type\nrequires an outlives annotation. Outlives annotations\n(e.g., `T: 'a`) are used to guarantee that all the data in T is valid\nfor at least the lifetime `'a`. This scenario most commonly\narises when the type contains an associated type reference\nlike `<T as SomeTrait<'a>>::Output`, as shown in this example:\n\n