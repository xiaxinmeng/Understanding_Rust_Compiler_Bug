\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs","byte_start":1242,"byte_end":1244,"line_start":41,"line_end":41,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"    let cell = Cell::new(&a);","highlight_start":26,"highlight_end":28}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs","byte_start":1532,"byte_end":1533,"line_start":49,"line_end":49,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"borrowed value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: `a` does not live long enough\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:41:26\n   |\nLL |     let cell = Cell::new(&a);\n   |                          ^^ borrowed value does not live long enough\n...\nLL | }\n   | - borrowed value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:41:20] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:41:20] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] 23    = note: number of external vids: 3
[00:41:20] 24    = note: where '_#1r: '_#0r
[00:41:20] 25 
[00:41:20] - error: argument requires that data must outlive free region `ReStatic`
[00:41:20] + error: unsatisfied lifetime constraints
[00:41:20] 28    |
[00:41:20] 28    |
[00:41:20] 29 LL | /     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
[00:41:20] 
[00:41:20] 32 LL | |         demand_y(x, y, x.get())
[00:41:20] 33 LL | |         //~^ WARNING not reporting region error due to nll
[00:41:20] 34 LL | |     });
[00:41:20] -    | |______^
[00:41:20] +    | |______^ argument requires that `'a` must outlive `'static`
[00:41:20] 37 note: No external requirements
[00:41:20] 38   --> $DIR/propagate-approximated-shorter-to-static-wrong-bound.rs:47:1
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/propagate-approximated-shorter-to-static-wrong-bound.stderr
[00:41:20] To update references, rerun the tests and pass the `--bless` flag
[00:41:20] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs`
[00:41:20] error: 1 errors occurred comparing output.
[00:41:20] status: exit code: 101
[00:41:20] status: exit code: 101
[00:41:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/auxiliary" "-A" "unused"
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] stderr:
[00:41:20] stderr:
[00:41:20] ------------------------------------------
[00:41:20] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1749,"byte_end":1772,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:51:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:41:20] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1603,"byte_end":1837,"line_start":48,"line_end":53,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 3","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#0r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:48:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |  _______________________________________________^\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ WARNING not reporting region error due to nll\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n           ]\n   = note: number of external vids: 3\n   = note: where '_#1r: '_#0r\n\n"}
[00:41:20] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1561,"byte_end":1838,"line_start":48,"line_end":53,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":5,"highlight_end":79},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":7}],"label":"argument requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsatisfied lifetime constraints\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:48:5\n   |\nLL | /     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ WARNING not reporting region error due to nll\nLL | |     });\n   | |______^ argument requires that `'a` must outlive `'static`\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1491,"byte_end":1841,"line_start":47,"line_end":54,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":1,"highlight_end":79},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:47:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |         // Only works if 'x: 'y:\n...  |\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]) with substs []\n\n"}
[00:41:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] 4 LL |         demand_y(x, y, x.get())
[00:41:20] 6 
[00:41:20] 6 
[00:41:20] - error: argument requires that data must outlive free region `ReFree(DefId(0/1:18 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]::{{closure}}[0]), BrAnon(2))`
[00:41:20] + error: unsatisfied lifetime constraints
[00:41:20] 9    |
[00:41:20] 9    |
[00:41:20] + LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
[00:41:20] +    |                                                ---------  - region `'1` appears in this argument
[00:41:20] +    |                                                |
[00:41:20] +    |                                                region `'2` appears in this argument
[00:41:20] + LL |         // Only works if 'x: 'y:
[00:41:20] 10 LL |         demand_y(x, y, x.get())
[00:41:20] -    |         ^^^^^^^^^^^^^^^^^^^^^^^
[00:41:20] +    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
[00:41:20] 13 note: No external requirements
[00:41:20] 14   --> $DIR/propagate-fail-to-approximate-longer-no-bounds.rs:45:47
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
[00:41:20] To update references, rerun the tests and pass the `--bless` flag
[00:41:20] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs`
[00:41:20] error: 1 errors occurred comparing output.
[00:41:20] status: exit code: 101
[00:41:20] status: exit code: 101
[00:41:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/auxiliary" "-A" "unused"
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] stderr:
[00:41:20] stderr:
[00:41:20] ------------------------------------------
[00:41:20] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1636,"byte_end":1659,"line_start":47,"line_end":47,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:47:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:41:20] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1587,"byte_end":1588,"line_start":45,"line_end":45,"column_start":59,"column_end":60,"is_primary":false,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":59,"highlight_end":60}],"label":"region `'1` appears in this argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1576,"byte_end":1585,"line_start":45,"line_end":45,"column_start":48,"column_end":57,"is_primary":false,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":48,"highlight_end":57}],"label":"region `'2` appears in this argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1636,"byte_end":1659,"line_start":47,"line_end":47,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":"argument requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsatisfied lifetime constraints\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:47:9\n   |\nLL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\n   |                                                ---------  - region `'1` appears in this argument\n   |                                                |\n   |                                                region `'2` appears in this argument\nLL |         // Only works if 'x: 'y:\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1575,"byte_end":1793,"line_start":45,"line_end":50,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":47,"highlight_end":66},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARN not reporting region error due to nll","highlight_start":1,"highlight_end":56},{"text":"        //~| ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:45:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\n   |  _______________________________________________^\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ WARN not reporting region error due to nll\nLL | |         //~| ERROR argument requires that data must outlive free region\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0, 't1, 't2> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>))\n           ]\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs","byte_start":1463,"byte_end":1797,"line_start":44,"line_end":51,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":1,"highlight_end":66},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARN not reporting region error due to nll","highlight_start":1,"highlight_end":56},{"text":"        //~| ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:44:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\n...  |\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_no_bounds[317d]::supply[0]) with substs []\n\n"}
[00:41:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] 4 LL |         demand_y(x, y, x.get())
[00:41:20] 6 
[00:41:20] 6 
[00:41:20] - error: free region `ReFree(DefId(0/1:18 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]::{{closure}}[0]), BrAnon(2))` does not outlive free region `ReFree(DefId(0/1:18 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]::{{closure}}[0]), BrAnon(4))`
[00:41:20] -   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:51:18
[00:41:20] + error: unsatisfied lifetime constraints
[00:41:20] +   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:51:9
[00:41:20] 9    |
[00:41:20] + LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
[00:41:20] +    |                                                ----------  ---------- region `'2` appears in this argument
[00:41:20] +    |                                                |
[00:41:20] +    |                                                region `'1` appears in this argument
[00:41:20] + LL |         // Only works if 'x: 'y:
[00:41:20] 10 LL |         demand_y(x, y, x.get())
[00:41:20] -    |                  ^
[00:41:20] +    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
[00:41:20] 13 note: No external requirements
[00:41:20] 14   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:49:47
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
[00:41:20] To update references, rerun the tests and pass the `--bless` flag
[00:41:20] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs`
[00:41:20] error: 1 errors occurred comparing output.
[00:41:20] status: exit code: 101
[00:41:20] status: exit code: 101
[00:41:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/auxiliary" "-A" "unused"
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] stderr:
[00:41:20] stderr:
[00:41:20] ------------------------------------------
[00:41:20] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1825,"byte_end":1848,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:51:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:41:20] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1752,"byte_end":1762,"line_start":49,"line_end":49,"column_start":48,"column_end":58,"is_primary":false,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":48,"highlight_end":58}],"label":"region `'1` appears in this argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1764,"byte_end":1774,"line_start":49,"line_end":49,"column_start":60,"column_end":70,"is_primary":false,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":60,"highlight_end":70}],"label":"region `'2` appears in this argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1825,"byte_end":1848,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":"argument requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsatisfied lifetime constraints\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:51:9\n   |\nLL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |                                                ----------  ---------- region `'2` appears in this argument\n   |                                                |\n   |                                                region `'1` appears in this argument\nLL |         // Only works if 'x: 'y:\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1751,"byte_end":1958,"line_start":49,"line_end":54,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARN not reporting region error due to nll","highlight_start":1,"highlight_end":56},{"text":"        //~| ERROR does not outlive free region","highlight_start":1,"highlight_end":48},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:49:47\n   |\nLL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\n   |  _______________________________________________^\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ WARN not reporting region error due to nll\nLL | |         //~| ERROR does not outlive free region\nLL | |     });\n   | |_____^\n   |\n   = note: defining type: DefId(0/1:18 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's, 't0, 't1, 't2, 't3> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't3)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) u32>))\n           ]\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs","byte_start":1639,"byte_end":1962,"line_start":48,"line_end":55,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":1,"highlight_end":79},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARN not reporting region error due to nll","highlight_start":1,"highlight_end":56},{"text":"        //~| ERROR does not outlive free region","highlight_start":1,"highlight_end":48},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:48:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\n...  |\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_fail_to_approximate_longer_wrong_bounds[317d]::supply[0]) with substs []\n\n"}
[00:41:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] 4 LL |     &*x
[00:41:20] 6 
[00:41:20] 6 
[00:41:20] - error: free region `ReFree(DefId(0/0:3 ~ region_lbr_named_does_not_outlive_static[317d]::foo[0]), BrNamed(crate0:DefIndex(1:9), 'a))` does not outlive free region `ReStatic`
[00:41:20] + error: unsatisfied lifetime constraints
[00:41:20] 9    |
[00:41:20] 9    |
[00:41:20] 10 LL |     &*x
[00:41:20] -    |     ^^^
[00:41:20] -    |     ^^^
[00:41:20] +    |     ^^^ free region requires that `'a` must outlive `'static`
[00:41:20] 13 error: aborting due to previous error
[00:41:20] 14 
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/region-lbr-named-does-not-outlive-static.stderr
[00:41:20] To update references, rerun the tests and pass the `--bless` flag
[00:41:20] To only update this specific test, also pass `--test-args nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs`
[00:41:20] error: 1 errors occurred comparing output.
[00:41:20] status: exit code: 101
[00:41:20] status: exit code: 101
[00:41:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/auxiliary" "-A" "unused"
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] stderr:
[00:41:20] stderr:
[00:41:20] ------------------------------------------
[00:41:20] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs","byte_start":817,"byte_end":820,"line_start":19,"line_end":19,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    &*x","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs:19:5\n   |\nLL |     &*x\n   |     ^^^\n\n"}
[00:41:20] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs","byte_start":817,"byte_end":820,"line_start":19,"line_end":19,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    &*x","highlight_start":5,"highlight_end":8}],"label":"free region requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsatisfied lifetime constraints\n  --> /checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs:19:5\n   |\nLL |     &*x\n   |     ^^^ free region requires that `'a` must outlive `'static`\n\n"}
[00:41:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/closure-requirements/return-wrong-bound-region.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] 4 LL |     expect_sig(|a, b| b); // ought to return `a`
[00:41:20] 6 
[00:41:20] 6 
[00:41:20] - error: free region `ReFree(DefId(0/1:9 ~ return_wrong_bound_region[317d]::test[0]::{{closure}}[0]), BrAnon(2))` does not outlive free region `ReFree(DefId(0/1:9 ~ return_wrong_bound_region[317d]::test[0]::{{closure}}[0]), BrAnon(1))`
[00:41:20] + error: unsatisfied lifetime constraints
[00:41:20] 8   --> $DIR/return-wrong-bound-region.rs:21:23
[00:41:20] 9    |
[00:41:20] 10 LL |     expect_sig(|a, b| b); // ought to return `a`
[00:41:20] -    |                       ^
[00:41:20] -    |                       ^
[00:41:20] +    |                 -  -  ^ free region requires that `'1` must outlive `'2`
[00:41:20] +    |                 |  |
[00:41:20] +    |                 |  region `'1` appears in this argument
[00:41:20] +    |                 region `'2` appears in this argument
[00:41:20] 13 note: No external requirements
[00:41:20] 14   --> $DIR/return-wrong-bound-region.rs:21:16
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/return-wrong-bound-region.stderr
[00:41:20] To update references, rerun the tests and pass the `--bless` flag
[00:41:20] To only update this specific test, also pass `--test-args nll/closure-requirements/return-wrong-bound-region.rs`
[00:41:20] error: 1 errors occurred comparing output.
[00:41:20] status: exit code: 101
[00:41:20] status: exit code: 101
[00:41:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/auxiliary" "-A" "unused"
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] stderr:
[00:41:20] stderr:
[00:41:20] ------------------------------------------
[00:41:20] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":743,"byte_end":744,"line_start":21,"line_end":21,"column_start":23,"column_end":24,"is_primary":true,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":23,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:21:23\n   |\nLL |     expect_sig(|a, b| b); // ought to return `a`\n   |                       ^\n\n"}
[00:41:20] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":740,"byte_end":741,"line_start":21,"line_end":21,"column_start":20,"column_end":21,"is_primary":false,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":20,"highlight_end":21}],"label":"region `'1` appears in this argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":737,"byte_end":738,"line_start":21,"line_end":21,"column_start":17,"column_end":18,"is_primary":false,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":17,"highlight_end":18}],"label":"region `'2` appears in this argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":743,"byte_end":744,"line_start":21,"line_end":21,"column_start":23,"column_end":24,"is_primary":true,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":23,"highlight_end":24}],"label":"free region requires that `'1` must outlive `'2`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsatisfied lifetime constraints\n  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:21:23\n   |\nLL |     expect_sig(|a, b| b); // ought to return `a`\n   |                 -  -  ^ free region requires that `'1` must outlive `'2`\n   |                 |  |\n   |                 |  region `'1` appears in this argument\n   |                 region `'2` appears in this argument\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":736,"byte_end":744,"line_start":21,"line_end":21,"column_start":16,"column_end":24,"is_primary":true,"text":[{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":16,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:9 ~ return_wrong_bound_region[317d]::test[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) i32)) -> &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:21:16\n   |\nLL |     expect_sig(|a, b| b); // ought to return `a`\n   |                ^^^^^^^^\n   |\n   = note: defining type: DefId(0/1:9 ~ return_wrong_bound_region[317d]::test[0]::{{closure}}[0]) with closure substs [\n               i16,\n               for<'r, 's> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) i32)) -> &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) i32\n           ]\n\n"}
[00:41:20] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs","byte_start":709,"byte_end":867,"line_start":20,"line_end":24,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn test() {","highlight_start":1,"highlight_end":12},{"text":"    expect_sig(|a, b| b); // ought to return `a`","highlight_start":1,"highlight_end":49},{"text":"    //~^ WARN not reporting region error due to nll","highlight_start":1,"highlight_end":52},{"text":"    //~| ERROR does not outlive free region","highlight_start":1,"highlight_end":44},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:3 ~ return_wrong_bound_region[317d]::test[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:20:1\n   |\nLL | / fn test() {\nLL | |     expect_sig(|a, b| b); // ought to return `a`\nLL | |     //~^ WARN not reporting region error due to nll\nLL | |     //~| ERROR does not outlive free region\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:3 ~ return_wrong_bound_region[317d]::test[0]) with substs []\n\n"}
[00:41:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/closure-requirements/return-wrong-bound-region.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/issue-48238.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] - error: free region `` does not outlive free region `'_#1r`
[00:41:20] -   --> $DIR/issue-48238.rs:21:21
[00:41:20] + error: internal compiler error: librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:77: can't make a name for free region '_#2r
[00:41:20] +   --> $DIR/issue-48238.rs:21:5
[00:41:20] 3    |
[00:41:20] 4 LL |     move || use_val(&orig); //~ ERROR free region `` does not outlive free region `'_#1r`
[00:41:20] +    |     ^^^^^^^^^^^^^^^^^^^^^^
[00:41:20] 6 
[00:41:20] 7 error: aborting due to previous error
[00:41:20] 8 
[00:41:20] 8 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/issue-48238.stderr
[00:41:20] To update references, rerun the tests and pass the `--bless` flag
[00:41:20] To only update this specific test, also pass `--test-args nll/issue-48238.rs`
[00:41:20] error: 1 errors occurred comparing output.
[00:41:20] status: exit code: 101
[00:41:20] status: exit code: 101
[00:41:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-48238.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/auxiliary" "-A" "unused"
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] stderr:
[00:41:20] stderr:
[00:41:20] ------------------------------------------
[00:41:20] {"message":"librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:77: can't make a name for free region '_#2r","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-48238.rs","byte_start":611,"byte_end":633,"line_start":21,"line_end":21,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    move || use_val(&orig); //~ ERROR free region `` does not outlive free region `'_#1r`","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:77: can't make a name for free region '_#2r\n  --> /checkout/src/test/ui/nll/issue-48238.rs:21:5\n   |\nLL |     move || use_val(&orig); //~ ERROR free region `` does not outlive free region `'_#1r`\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:41:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:41:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:41:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:41:20] 
[00:41:20] note: the compiler unexpectedly panicked. this is a bug.
[00:41:20] 
[00:41:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:41:20] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:41:20] 
[00:41:20] 
[00:41:20] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:41:20] 
[00:41:20] ------------------------------------------
[00:41:20] 
[00:41:20] thread '[ui] ui/nll/issue-48238.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] thread '[ui] ui/nll/issue-48238.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:41:20] 
[00:41:20] ---- [ui] ui/nll/issue-50716.rs stdout ----
[00:41:20] diff of stderr:
[00:41:20] 
[00:41:20] - error: assignment requires that data must outlive free region `'static`
[00:41:20] + error: unsatisfied lifetime constraints
[00:41:20] 3    |
[00:41:20] 3    |
[00:41:20] 4 LL |     let _x = *s; //~ ERROR assignment requires that data must outlive free region `'static`
[00:41:20] -    |              ^^
[00:41:20] -    |              ^^
[00:41:20] +    |              ^^ assignment requires that `'a` must outlive `'static`
[00:41:20] 7 error: aborting due to previous error
[00:41:20] 8 
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] The actual stderr differed from the expected stderr.
[00:41:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/issue-50716.stderr
---
[00:41:20] test result: FAILED. 1522 passed; 14 failed; 5 ignored; 0 measured; 0 filtered out
[00:41:20] 
[00:41:20] 
[00:41:20] 
[00:41:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:41:20] 
[00:41:20] 
[00:41:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:41:20] Build completed unsuccessfully in 0:02:03
[00:41:20] Build completed unsuccessfully in 0:02:03
[00:41:20] make: *** [check] Error 1
[00:41:20] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c5604d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
