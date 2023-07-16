plain
[00:48:04] ....................................................................................................
[00:48:08] ....................................................................................................
[00:48:11] .........................................i..........................................................
[00:48:14] ...........................................................................................i.i..ii..
[00:48:18] ..................................................................FF................................
[00:48:21] ..F......................FF.F..........................................................i............
[00:48:27] ....................................................................................................
[00:48:30] ....................................................................................................
[00:48:30] ....................................................................................................
[00:48:33] ..............F.....................................................................................
[00:48:39] ....................................................................................................
[00:48:42] ....................................................................................................
[00:48:46] ...................................................i................................................
[00:48:50] ....................................................................................................
[00:48:50] ....................................................................................................
[00:48:53] ....................................................................................................
[00:48:55] ..............................................................................................i.....
[00:48:57] .......................................................
[00:48:57] failures:
[00:48:57] 
[00:48:57] ---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs stdout ----
[00:48:57] 
[00:48:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:57] status: exit code: 101
[00:48:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-eporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1696,"byte_end":1719,"line_start":51,"line_end":51,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        demand_y(x, y, x.get())","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:51:9\n   |\nLL |         demand_y(x, y, x.get())\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:48:57] {"message":"External requirements","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs","byte_start":1603,"byte_end":1784,"line_start":48,"line_end":53,"column_start":47,"column_end":6,"is_primary":true,"text":[{"text":"    establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {","highlight_start":47,"highlight_end":79},{"text":"        //~^ ERROR","highlight_start":1,"highlight_end":19},{"text":"        // Only works if 'x: 'y:","highlight_start":1,"highlight_end":33},{"text":"        demand_y(x, y, x.get())","highlight_start":1,"highlight_end":32},{"text":"        //~^ WARNING not reporting region error due to nll","highlight_start":1,"highlight_end":59},{"text":"    });","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacem/runtest.rs:3189:9
[00:48:57] ---- [ui] ui/nll/mir_check_cast_unsafe_fn.rs stdout ----
[00:48:57] diff of stderr:
[00:48:57] 
[00:48:57] 5    |                                ^
[00:48:57] 5    |                                ^
[00:48:57] 6 
[00:48:57] 7 error: unsatisfied lifetime constraints
[00:48:57] -   --> $DIR/mir_check_cast_unsafe_fn.rs:20:14
[00:48:57] +   --> $DIR/mir_check_cast_unsafe_fn.rs:18:32
[00:48:57] 9    |
[00:48:57] 10 LL | fn bar<'a>(input: &'a u32, f: fn(&'a u32) -> &'a u32) -> &'static u32 {
[00:48:57] 11    |        -- lifetime `'a` defined here
[00:48:57] 12 ...
[00:48:57] 12 ...
[00:48:57] - LL |     unsafe { g(input) }
[00:48:57] -    |              ^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[00:48:57] + LL |     let g: unsafe fn(_) -> _ = f;
[00:48:57] +    |                                ^ cast requires that `'a` must outlive `'static`
[00:48:57] 16 error: aborting due to previous error
[00:48:57] 17 
[00:48:57] 
[00:48:57] 
[00:48:57] 
[00:48:57] The actual stderr differed from the expected stderr.
[00:48:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/mir_check_cast_unsafe_fn.stderr
[00:48:57] To update references, rerun the tests and pass the `--bless` flag
[00:48:57] To only update this specific test, also pass `--test-args nll/mir_check_cast_unsafe_fn.rs`
[00:48:57] error: 1 errors occurred comparing output.
[00:48:57] status: exit code: 1
[00:48:57] status: exit code: 1
[00:48:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsafe_fn/auxiliary" "-A" "unused"
[00:48:57] ------------------------------------------
[00:48:57] 
[00:48:57] ------------------------------------------
[00:48:57] stderr:
[00:48:57] stderr:
[00:48:57] ------------------------------------------
[00:48:57] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs","byte_start":761,"byte_end":762,"line_start":18,"line_end":18,"column_start":32,"column_end":33,"is_primary":true,"text":[{"text":"    let g: unsafe fn(_) -> _ = f;","highlight_start":32,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs:18:32\n   |\nLL |     let g: unsafe fn(_) -> _ = f;\n   |                                ^\n\n"}
[00:48:57] {"message":"unsatisfied lifetime constraints","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/mir_check_cast_unsafe_fn.rs","byte_start":530,"byte_end":532,"line_start":15,"line_end":15,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn bar<'a>(input: &'a u32, s:17:46
[00:48:57] +   --> $DIR/mir_check_cast_unsize.rs:19:5
[00:48:57] 9    |
[00:48:57] - LL |   fn bar<'a>(x: &'a u32) -> &'static dyn Debug {
[00:48:57] -    |  ________--____________________________________^
[00:48:57] -    | |        lifetime `'a` defined here
[00:48:57] -    | |        lifetime `'a` defined here
[00:48:57] - LL | |     //~^ ERROR unsatisfied lifetime constraints
[00:48:57] - LL | |     x
[00:48:57] - LL | |     //~^ WARNING not reporting region error due to nll
[00:48:57] - LL | | }
[00:48:57] -    | |_^ returning this value requires that `'a` must outlive `'static`
[00:48:57] + LL | fn bar<'a>(x: &'a u32) -> &'static dyn Debug {
[00:48:57] +    |        -- lifetime `'a` defined here
[00:48:57] + LL |     //~^ ERROR unsatisfied lifetime constraints
[00:48:57] + LL |     x
[00:48:57] +    |     ^ cast requires that `'a` must outlive `'static`
[00:48:57] 20 error: aborting due to previous error
[00:48:57] 21 
[00:48:57] 
[00:48:57] 
[00:48:57] 
[00:48:57] The actual stderr differed from the expected stderr.
[00:48:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_unsize/mir_check_cast_unsize.stderr
[00:48:57] To update references, rerun the tests and pass the `--bless` flag
[00:48:57] To only update this specific test, also pass `--test-args nll/mir_check_cast_unsize.rs`
[00:48:57] error: 1 errors occurred comparing output.
[00:48:57] status: exit code: 1
[00:48:57] status: exit code: 1
[00:48:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_unsize.rs" "--target=x86_64-unknown-linux-gnu" "--error-fohe signature and the body must be made to match. Typically,\nthis is done by updating the function signature. So, in this case, we change\nthe type of `y` to `&'a i32`, like so:\n\n