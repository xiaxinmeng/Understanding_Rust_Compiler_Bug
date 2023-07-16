plain
[00:47:19] ....................................................................................................
[00:47:22] ....................................................................................................
[00:47:25] ....................................................................................................
[00:47:28] ....................................................................................................
[00:47:31] ....................................................i........................F.FFFF.F...............
[00:47:35] .........F.....................FF.FFFF.FF.....i.....................................................
[00:47:41] ....................................................................................................
[00:47:47] ...................................................i....
[00:47:47] failures:
[00:47:47] 
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/closure-requirements/propagate-approximated-ref.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/auxiliary" "-A" "unused"
[00:47:47] stdout:
[00:47:47] --------------_end":33},{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":82},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_approximated_ref[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:53:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |  _______________________________________________^\nLL | |         //~^ ERROR lifetime mismatch\nLL | |\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_approximated_ref[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n           ]\n   = note: number of external vids: 5\n   = note: where '_#1r: '_#2r\n\n"}
[00:47:47]   left: `3`,
[00:47:47]   left: `3`,
[00:47:47]  right: `5`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-approximated-ref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-approximated-ref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1609,"byte_end":1632,"line_start":49,"line_end":49,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:49:9\n   |\nLL |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1528,"byte_end":1688,"line_start":45,"line_end":50,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":47,"highlight_end":66},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"","highlight_____________^\nLL | |         //~^ ERROR\nLL | |\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_approximated_shorter_to_static_no_bound[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0, 't1, 't2> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) u32>))\n           ]\n   = note: number of external vids: 4\n   = note: where '_#1r: '_#0r\n\n"}
[00:47:47]   left: `2`,
[00:47:47]   left: `2`,
[00:47:47]  right: `4`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] 
[00:47:47] note: rustc 1.29.0-dev ru---------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1696,"byte_end":1719,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:51:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1603,"byte_end":1784,"line_start":48,"line_end":53,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_enIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/closure-requirements/propagate-approximated-val.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"mess\n               for<'r, 's> extern \"rust-call\" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>))\n           ]\n   = note: number of external vids: 5\n   = note: where '_#1r: '_#2r\n\n"}
[00:47:47]   left: `3`,
[00:47:47]   left: `3`,
[00:47:47]  right: `5`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-approximated-val.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-approximated-val.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/closure-requirements/propagate-despite-same-frequirements/propagate-despite-same-free-region.rs:54:21\n   |\nLL |             let p = x.get();\n   |                     ^^^^^^^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-despite-same-free-region.rs","byte_start":1620,"byte_end":1758,"line_start":52,"line_end":56,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        |_outlives1, _outlives2, x, y| {","highlight_start":9,"highlight_end":41},{"text":"            // Only works if 'x: 'y:","highlight_start":1,"highlight_end":37},{"text":"            let p = x.get();","highlight_start":1,"highlight_end":29},{"text":"            demand_y(x, y, p)","highlight_start":1,"highlight_end":30},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:16 ~ propagate_despite_same_free_region[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's> extern \"rust-call\" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"ch report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-despite-same-free-region.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/closure-requirements/propagate-from-trait-match.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-from-trait-match.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-from-trait-match/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-from-trait-match/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] 
[00:47:47 reporting region error due to nll\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:16 ~ propagate_from_trait_match[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((T,))\n           ]\n   = note: number of external vids: 3\n   = note: where T: '_#1r\n\n"}
[00:47:47]   left: `2`,
[00:47:47]   left: `2`,
[00:47:47]  right: `3`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-from-trait-match.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/closure-requirements/propagate-from-trait-match.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/issue-50716.rs stdout ----
[00:47:47] diff of stderr:
[00:47:47] 
[00:47:47] - error: borrowed data escapes outside of closure
[00:47:47] + error: unsatisfied lifror-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-implied-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-implied-bounds/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-implied-bounds.rs","byte_start":1198,"byte_end":1205,"line_start":45,"line_end":45,"column_start":36,"column_end":43,"is_primary":true,"text":[{"text":"    twice(value, |value_ref, item| invoke2(value_ref, item));","highlight_start":36,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-implied-bounds.rs:45:36\n   |\nLL |     twice(value, |value_ref, item| invoke2(value_ref, item));\n   |                                    ^^^^^^^\n\n"}
[00:47:47]   left: `1`,
[00:47:47]   left: `1`,
[00:47:47]  right: `2`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/ty-outlives/projection-implied-bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/ty-outlives/projection-implied-bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/ty-outlives/projection-no-regions-closure.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-no-regions-closure/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs","byte_start":1046,"byte_end":1064,"line_start":35,"line_end":35,"column_start":31,"column_end":49,"is_primary":true,"text":[{"text":"    with_signature(x, |mut y| Box::new(y.next()))","highlight_start":31,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs:35:31\n   |\nLL |     with_signature(x, |mut y| Box::new(y.next()))\n   |                               ^^^^^^^^^^^^^^^^^^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-no-regions-closure.rs","byte_start":1522,"byte_end":1540,"line_start":53,"line_end":53,"column_start":31,"column_end":49,"is_primary":true,"text":[{"text":"    with_signature(x, |mut y| Box::new(y.next()))","highlight_start":31,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  -->heckout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs","byte_start":1582,"byte_end":1589,"line_start":55,"line_end":55,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:55:39\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                                       ^^^^^^^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"waon-one-region-closure.rs","byte_start":1572,"byte_end":1598,"line_start":55,"line_end":55,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:19 ~ projection_one_region_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where T: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:55:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:19 ~ projection_one_region_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n           ]\n   = note: number of external vids: 5\n   = note: where T: '_#2r\n   = note: where '_#1r: '_#2r\n\n"}
[00:47:47]   left: `3`,
[00:47:47]   left: `3`,
[00:47:47]  right: `5`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/ty-outlives/projection-one-region-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/ty-outlives/projection-one-region-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs","byte_start":1321,"byte_end":1328,"line_start":47,"line_end":47,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:47:39\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                                       ^^^^^^^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs","byte_start":1572,"byte_end":1579,"line_start":58,"line_end":58,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:58:39\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                                       ^^^^^^^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs","byte_start":2424,"byte_end":2431,"line_start":79,"line_end":79,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:79:39\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                                       ^^^^^^^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs","byte_start":1311,"byte_end":1337,"line_start":47,"line_end":47,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:19 ~ projection_one_region_trait_bound_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:47:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:19 ~ projection_one_region_trait_bound_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::cell::Cell<&'_#2r ()>, T))\n           ]\n   = note: number of external vids: 5\n   = note: where '_#1r: '_#2r\n\n"}
[00:47:47]   left: `3`,
[00:47:47]   left: `3`,
[00:47:47]  right: `5`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] note: Run with `RUST_BACKTRAection-two-region-trait-bound-closure/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":1378,"byte_end":1385,"line_start":48,"line_end":48,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:48:39\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                                       ^^^^^^^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":1721,"byte_end":1728,"line_start":59,"line_end":59,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered"ng: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:108:39\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                                       ^^^^^^^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs","byte_start":1368,"byte_end":1394,"line_start":48,"line_end":48,"column_start":29,"column_end":55,"is_primary":true,"text":[{"text":"    with_signature(cell, t, |cell, t| require(cell, t));","highlight_start":29,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:22 ~ projection_two_region_trait_bound_closure[317d]::no_relationships_late[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    '_#2r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#3r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 6","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where <T as Anything<ReClosureBound('_#1r), ReClosureBound('_#2r)>>::AssocType: '_#3r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/projection-two-region-trait-bound-closure.rs:48:29\n   |\nLL |     with_signature(cell, t, |cell, t| require(cell, t));\n   |                             ^^^^^^^^^^^^^^^^^^^-
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs","byte_start":972,"byte_end":984,"line_start":34,"line_end":34,"column_start":31,"column_end":43,"is_primary":true,"text":[{"text":"    twice(cell, value, |a, b| invoke(a, b));","highlight_start":31,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-ou               T,\n               i16,\n               for<'r, 's> extern \"rust-call\" fn((std::option::Option<std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) ()>>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) T))\n           ]\n   = note: number of external vids: 3\n   = note: where T: '_#1r\n\n"}
[00:47:47]   left: `2`,
[00:47:47]   left: `2`,
[00:47:47]  right: `3`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/ty-outlives/ty-param-closure-approximate-lower-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] sta     ^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs","byte_start":1528,"byte_end":1529,"line_start":52,"line_end":52,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:52:5\n   |\nLL |     x\n   |     ^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs","byte_start":1221,"byte_end":1226,"line_start":36,"line_end":36,"column_start":23,"column_end":28,"is_primary":true,"text":[{"text":"    with_signature(x, |y| y)","highlight_start":23,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:14 ~ ty_param_closure_outlives_from_return_type[317d]::no_region[0]::{{closure}}[0]) with closure substs [\n    '_#1r,\n    T,\n    i32,\n    extern \"rust-call\" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn std::fmt::Debug + '_#2r)>\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where T: '_#2r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs:36:23\n   |\nLL |     with_signature(x, |y| y)\n   |                       ^^^^^\n   |\n   = note: defining type: DefId(0/1:14 ~ ty_param_closure_outlives_from_return_type[317d]::no_region[0]::{{closure}}[0]) with closure substs [\n               '_#1r,\n               T,\n               i32,\n               extern \"rust-call\" fn((std::boxed::Box<T>,)) -> std::boxed::Box<(dyn std::fmt::Debug + '_#2r)>\n           ]\n   = note: number of external vids: 4\n   = note: where T: '_#2r\n\n"}
[00:47:47]   left: `3`,
[00:47:47]   left: `3`,
[00:47:47]  right: `4`: index vec had unexpected number of variables', librustc_mir/borrow_check/nll/universal_regions.rs:280:9
[00:47:47] 
[00:47:47] error: internal compiler error: unexpected panic
[00:47:47] 
[00:47:47] 
[00:47:47] note: the compiler unexpectedly panicked. this is a bug.
[00:47:47] 
[00:47:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:47] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:47:47] 
[00:47:47] 
[00:47:47] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] thread '[ui] ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] thread '[ui] ui/nll/ty-outlives/ty-param-closure-outlives-from-return-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:47:47] 
[00:47:47] ---- [ui] ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs stdout ----
[00:47:47] 
[00:47:47] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:47] status: exit code: 101
[00:47:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause/auxiliary" "-A" "unused"
[00:47:47] ------------------------------------------
[00:47:47] 
[00:47:47] ------------------------------------------
[00:47:47] stderr:
[00:47:47] stderr:
[00:47:47] ------------------------------------------
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs","byte_start":1373,"byte_end":1380,"line_start":44,"line_end":44,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        require(&x, &y)","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:44:9\n   |\nLL |         require(&x, &y)\n   |         ^^^^^^^\n\n"}
[00:47:47] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs","byte_start":2485,"byte_end":2492,"line_start":78,"line_end":78,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        require(&x, &y)","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:78:9\n   |\nLL |         require(&x, &y)\n   |         ^^^^^^^\n\n"}
[00:47:47] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs","byte_start":1011,"byte_end":1453,"line_start":37,"line_end":46,"column_start":26,"column_end":6,"is_primary":true,"text":[{"text":"    with_signature(a, b, |x, y| {","highlight_start":26,"highlight_end":34},{"text":"        //~^ ERROR the parameter type `T` may not live long enough","highlight_start":1,"highlight_end":67},{"text":"        //","highlight_start":1,"highlight_end":11},{"text":"        // See `correct_region`, which explains the point of this","highlight_start":1,"highlight_end":66},{"text":"        // test.  The only difference is that, in the case of this","highlight_start":1,"highlight_end":67},{"text":"        // function, there is no where clause *anywhere*, and hence we","highlight_start":1,"highlight_end":71},{"text":"        // get an error (but reported by the closure creator).","highlight_start":1,"highlight_end":63},{"text":"        require(&x, &y)","highlight_start":1,"highlight_end":24},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    })","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:16 ~ ty_param_closure_outlives_from_where_clause[317d]::no_region[0]::{{closure}}[0]) with closure substs [\n    T,\n    i32,\n    extern \"rust-call\" fn((std::cell::Cell<&'_#1r ()>, T))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where T: '_#1r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/ty-outlives/ty-param-closure-outlives-from-where-clause.rs:37:26\n   |\nLL |       with_signature(a, b, |x, y| {\n   |  __________________________^\nLL | |         //~^ ERROR the parameter type `T` may not live long enough\nLL | |         //\nLL | |         // See `correct_regiopagate-approximated-ref.rs
[00:47:47]     [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs
[00:47:47]     [ui] ui/nll/closure-requirements/propagate-approximated-val.rs
[00:47:47]     [ui] ui/nll/closure-requirements/propagate-despite-same-free-region.rs
[00:47:47]     [ui] ui/nll/closure-requirements/propagate-from-trait-match.rs
---
[00:47:47] test result: FAILED. 2134 passed; 15 failed; 7 ignored; 0 measured; 0 filtered out
[00:47:47] 
[00:47:47] 
[00:47:47] 
[00:47:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-pathcompiler-rt/objects
33908 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
33908 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
33884 ./src/llvm-emscripten/lib/Target
33804 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build
