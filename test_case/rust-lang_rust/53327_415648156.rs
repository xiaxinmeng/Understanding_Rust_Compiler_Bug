plain
[00:45:10] ....................................................................................................
[00:45:13] ....................................................................................................
[00:45:16] .......................................i............................................................
[00:45:19] .........................................................................................i.i..ii....
[00:45:22] ................................................................FF..................................
[00:45:25] F....................................................................................i..............
[00:45:31] ....................................................................................................
[00:45:33] ....................................................................................................
[00:45:33] ....................................................................................................
[00:45:36] ............F.......................................................................................
[00:45:43] ....................................................................................................
[00:45:45] ....................................................................................................
[00:45:49] .................................................i..................................................
[00:45:52] ....................................................................................................
[00:45:52] ....................................................................................................
st/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/auxiliary" "-A" "unused"
[00:45:59] stdout:
[00:45:59] ------------------------------------------
[00:45:59] 
[00:45:59] ------------------------------------------
[00:45:59] stderr:
[00:45:59] ------------------------------------------
[00:45:59] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1609,"byte_end":1632,"line_start":49,"line_end":49,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:49:9\n   |\nLL |         demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:45:59] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs","byte_start":1528,"byte_end":1688,"line_start":45,"line_end":50,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {","highlight_start":47,"highlight_end":66},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get()) //~ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":82},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"defining type: DefId(0/1:18 ~ propagate_approximated_shorter_to_static_no_bound[317d]::supply[0]::{{closure}}[0]) with closure substs [\n    i16,\n    for<'r, 's, 't0, 't1, 't2> extern \"rust-call\" fn((&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't0)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 's)) u32>, &ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't1)) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 't2)) u32>))\n]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"number of external vids: 4","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"where '_#1r: '_#0r","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"note: External requirements\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:45:47\n   |\nLL |       establish_relat64-unknown-linux-gnu
[00:45:59] 
[00:45:59] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z verbose -C prefer-dynamic -C rpath
[00:45:59] 
[00:45:59] ------------------------------------------
[00:45:59] 
[00:45:59] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:45:59] thread '[ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:45:59] 
[00:45:59] ---- [ui] ui/nll/issue-50716.rs stdout ----
[00:45:59] 
[00:45:59] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:45:59] status: exit code: 101
[00:45:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-50716.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/auxiliary" "-A" "unused"
[00:45:59] ------------------------------------------
[00:45:59] 
[00:45:59] ------------------------------------------
[00:45:59] stderr:
[00:45:59] stderr:
[00:45:59] ------------------------------------------
[00:45:59] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
[00:45:59] 
[00:45:59] error: internal compiler error: unexpected panic
[00:45:59] 
[00:45:59] 
[00:45:59] note:s/regions-static-bound.rs:19:5\n   |\nLL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {\n   |                        -- lifetime `'a` defined here\nLL |     t //[ll]~ ERROR E0312\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:45:59] {"message":"explicit lifetime required in the type of `u`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n