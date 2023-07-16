plain
[00:45:41] ....................................................................................................
[00:45:46] ....................................................................................................
[00:45:52] ....................................................................................................
[00:45:57] ....................................................................................................
[00:46:04] .........................i........................FF..........................F.....................
[00:46:14] ....................................................................................................
[00:46:20] ....................................................................................................
[00:46:27] .................................................i..................................................
[00:46:29] ..................................
[00:46:29] ..................................
[00:46:29] failures:
[00:46:29] 
[00:46:29] ---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs stdout ----
[00:46:29] diff of stderr:
[00:46:29] 
[00:46:29] 32 LL | |         // Only works if 'x: 'y:
[00:46:29] 33 LL | |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll
[00:46:29] 34 LL | |     });
[00:46:29] +    | |______^
[00:46:29] 36 
[00:46:29] 37 note: No external requirements
[00:46:29] 38   --> $DIR/propagate-approximated-shorter-to-static-no-bound.rs:44:1
[00:46:29] 38   --> $DIR/propagate-approximated-shorter-to-static-no-bound.rs:44:1
[00:46:29] 
[00:46:29] 
[00:46:29] The actual stderr differed from the expected stderr.
[00:46:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/propagate-approximated-shorter-to-static-no-bound.stderr
[00:46:29] To update references, rerun the tests and pass the `--bless` flag
[00:46:29] To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs`
[00:46:29] error: 1 errors occurred comparing output.
[00:46:29] status: exit code: 101
[00:46:29] status: exit code: 101
[00:46:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/auxiliary" "-A" "unused"
[00:46:29] ------------------------------------------
[00:46:29] 
[00:46:29] ------------------------------------------
[00:46:29] stderr:
[00:46:29] stderr:
[00:46:29] ------------------------------------------
[00:46:29] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1662,"byte_end":1685,"line_start":49,"line_end":49,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorll<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) u32>))\n           ]\n   = note: number of external vids: 2\n   = note: where '_#1r: '_#0r\n\n"}
[00:46:29] {"message":"argument requires that data must outlive free region `ReStatic`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1486,"byte_end":1742,"line_start":45,"line_end":50,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":5,"highlight_end":66},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":82},{"text":"    });","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: argument requires that data must outlive free region `ReStatic`\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:45:5\n   |\nLL | /     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll\nLL | |     });\n   | |______^\n\n"}
[00:46:29] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1416,"byte_end":1745,"line_start":44,"line_end":51,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":1,"highlight_end":66},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":82},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_approximated_shorter_to_static_no_bound[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:44:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |\n...  |\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_approximated_shorter_to_static_no_bound[317d]::supply[0]) with substs []\n\n"}
[00:46:29] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:29] ------------------------------------------
[00:46:29] 
[00:46:29] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:46:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:29] 
[00:46:29] ---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs stdout ----
[00:46:29] diff of stderr:
[00:46:29] 
[00:46:29] 32 LL | |         demand_y(x, y, x.get())
[00:46:29] 33 LL | |         //~^ WARNING not reporting region error due to nll
[00:46:29] 34 LL | |     });
[00:46:29] +    | |______^
[00:46:29] 36 
[00:46:29] 37 note: No external requirements
[00:46:29] 38   --> $DIR/propagate-approximated-shorter-to-static-wrong-bound.rs:47:1
[00:46:29] 38   --> $DIR/propagate-approximated-shorter-to-static-wrong-bound.rs:47:1
[00:46:29] 
[00:46:29] 
[00:46:29] The actual stderr differed from the expected stderr.
[00:46:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/propagate-approximary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:51:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:29] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1603,"byte_end":1837,"line_start":48,"line_end":53,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<3},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: argument requires that data must outlive free region `ReStatic`\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:48:5\n   |\nLL | /     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |         // Only works if 'x: 'y:\nLL | |         demand_y(x, y, x.get())\nLL | |         //~^ WARNING not reporting region error due to nll\nLL | |     });\n   | |______^\n\n"}
[00:46:29] {"message":"No external requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1491,"byte_end":1841,"line_start":47,"line_end":54,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {","highlight_start":1,"highlight_end":66},{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":1,"highlight_end":79},{"text":"        //~^ ERROR argument requires that data must outlive free region","highlight_start":1,"highlight_end":72},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/0:6 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]) with substs []","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: No external requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:47:1\n   |\nLL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {\nLL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {\nLL | |         //~^ ERROR argument requires that data must outlive free region\nLL | |         // Only works if 'x: 'y:\n...  |\nLL | |     });\nLL | | }\n   | |_^\n   |\n   = note: defining type: DefId(0/0:6 ~ propagate_approximated_shorter_to_static_wrong_bound[317d]::supply[0]) with substs []\n\n"}
[00:46:29] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:29] ------------------------------------------
[00:46:29] 
[00:46:29] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic00:46:29] ------------------------------------------
[00:46:29] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic00:46:29] ------------------------------------------
[00:46:29] {"message":"assignment requires that data must outlive free region `'static`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-50716.rs","byte_start":751,"byte_end":753,"line_start":25,"line_end":25,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"    let _x = *s; //~ ERROR free region `'a` does not outlive free region `'static`","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: assignment requires that data must outlive free region `'static`\n  --> /checkout/src/test/ui/nll/issue-50716.rs:25:14\n   |\nLL |     let _x = *s; //~ ERROR free region `'a` does not outlive free region `'static`\n   |              ^^\n\n"}
[00:46:29] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:29] ------------------------------------------
[00:46:29] 
[00:46:29] thread '[ui] ui/nll/issue-50716.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:46:29] 
---
144088 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
144084 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
138748 ./obj/build/bootstrap/debug/incremental
124176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
124172 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f2g5m2h0cs-1nabiy0-38adk87dv7782
107256 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
103608
