\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2378,"byte_end":2380,"line_start":100,"line_end":100,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"        value1: &x, //~ ERROR","highlight_start":17,"highlight_end":19}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2420,"byte_end":2421,"line_start":103,"line_end":103,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`x` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2330,"byte_end":2350,"line_start":99,"line_end":99,"column_start":42,"column_end":62,"is_primary":false,"text":[{"text":"    let Double { value1: _, value2: _ }: Double<&'static u32> = Double {","highlight_start":42,"highlight_end":62}],"label":"type annotation requires that `x` is borrowed for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `x` does not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:100:17\n   |\nLL |     let Double { value1: _, value2: _ }: Double<&'static u32> = Double {\n   |                                          -------------------- type annotation requires that `x` is borrowed for `'static`\nLL |         value1: &x, //~ ERROR\n   |                 ^^ borrowed value does not live long enough\n...\nLL | }\n   | - `x` dropped here while still borrowed\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2465,"byte_end":2467,"line_start":105,"line_end":105,"column_start":43,"column_end":45,"is_primary":false,"text":[{"text":"fn static_to_a_to_static_through_variable<'a>(x: &'a u32) -> &'static u32 {","highlight_start":43,"highlight_end":45}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2535,"byte_end":2536,"line_start":113,"line_end":113,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:113:5\n   |\nLL | fn static_to_a_to_static_through_variable<'a>(x: &'a u32) -> &'static u32 {\n   |                                           -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2589,"byte_end":2591,"line_start":116,"line_end":116,"column_start":40,"column_end":42,"is_primary":false,"text":[{"text":"fn static_to_a_to_static_through_tuple<'a>(x: &'a u32) -> &'static u32 {","highlight_start":40,"highlight_end":42}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2679,"byte_end":2680,"line_start":125,"line_end":125,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:125:5\n   |\nLL | fn static_to_a_to_static_through_tuple<'a>(x: &'a u32) -> &'static u32 {\n   |                                        -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2734,"byte_end":2736,"line_start":128,"line_end":128,"column_start":41,"column_end":43,"is_primary":false,"text":[{"text":"fn static_to_a_to_static_through_struct<'a>(_x: &'a u32) -> &'static u32 {","highlight_start":41,"highlight_end":43}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2843,"byte_end":2844,"line_start":130,"line_end":130,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:130:5\n   |\nLL | fn static_to_a_to_static_through_struct<'a>(_x: &'a u32) -> &'static u32 {\n   |                                         -- lifetime `'a` defined here\nLL |     let Single { value: y }: Single<&'a u32> = Single { value: &22 };\nLL |     y //~ ERROR\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2885,"byte_end":2887,"line_start":133,"line_end":133,"column_start":28,"column_end":30,"is_primary":false,"text":[{"text":"fn a_to_static_then_static<'a>(x: &'a u32) -> &'static u32 {","highlight_start":28,"highlight_end":30}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/patterns.rs","byte_start":2936,"byte_end":2955,"line_start":134,"line_end":134,"column_start":18,"column_end":37,"is_primary":true,"text":[{"text":"    let (y, _z): (&'static u32, u32) = (x, 44); //~ ERROR","highlight_start":18,"highlight_end":37}],"label":"type annotation requires that `[00:48:48] 4 LL | fn bar<'a, 'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/where_clauses_in_functions.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/where_clauses_in_functions.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/where_clauses_in_functions.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/where_clauses_in_functions.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/where_clauses_in_functions.rs","byte_start":625,"byte_end":627,"line_start":22,"line_end":22,"column due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/where_clauses_in_functions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/nll/where_clauses_in_structs.rs stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/where_clauses_in_structs.rs:23:11
[00:48:48] 3    |
[00:48:48] 4 LL | fn bar<'a, 'b>(x: Cell<&'a u32>, y: Cell<&'b u32>) {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_structs/where_clauses_in_structs.stderr
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_structs/where_clauses_in_structs.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args nll/where_clauses_in_structs.rs`
[00:48:48] error: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/where_clauses_in_structs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_structs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/b_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unable to prove that references are valid\n  --> /checkout/src/test/ui/nll/where_clauses_in_structs.rs:23:11\n   |\nLL | fn bar<'a, 'b>(x: Cell<&'a u32>, y: Cell<&'b u32>) {\n   |        --  -- lifetime `'b` defined here\n   |        |\n   |        lifetime `'a` defined here\nLL |     Foo { x, y };\n   |           ^ requires that `'a` must outlive `'b`\n\n"}
[00:48:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] thread '[ui] ui/nll/where_clauses_in_structs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:48] 
[00:48:48] 
[00:48:48] ---- [ui] ui/regions/regions-static-bound.rs#nll stdout ----
[00:48:48] diff of stderr:
[00:48:48] 
[00:48:48] - error: unsatisfied lifetime constraints
[00:48:48] + error: unable to prove that references are valid
[00:48:48] 2   --> $DIR/regions-static-bound.rs:19:5
[00:48:48] 3    |
[00:48:48] 4 LL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
[00:48:48] 
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] The actual stderr differed from the expected stderr.
[00:48:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.nll/regions-static-bound.nll.stderr
[00:48:48] To update references, rerun the tests and pass the `--bless` flag
[00:48:48] To only update this specific test, also pass `--test-args regions/regions-static-bound.rs`
[00:48:48] 
[00:48:48] error in revision `nll`: 1 errors occurred comparing output.
[00:48:48] status: exit code: 1
[00:48:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-static-bound.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.nll/auxiliary" "-A" "unused"
[00:48:48] ------------------------------------------
[00:48:48] 
[00:48:48] ------------------------------------------
[00:48:48] stderr:
[00:48:48] stderr:
[00:48:48] ------------------------------------------
[00:48:48] {"message":"unable to prove that references are valid","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-static-bound.rs","byte_start":714,"byte_end":716,"line_start":18,"line_end":18,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {","highlight_start":24,"highlight_end":26}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/regions/regions-static-bound.rs","byte_start":768,"byte_end":769,"line_start":19,"line_end":19,"column_start":5,"columnE0621]\n   |     ^^^^^^^^^^^^^ lifetime `'static` required\n\n"}
[00:48:48] {"message":"explicit lifetime required in the type of `v`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n