plain
[00:45:39] ......i.............................................................................................
[00:45:42] ............ii.iii..................................................................................
[00:45:45] ....................................................................................................
[00:45:47] ....................................................................................................
[00:45:49] ......................................................................................F.............
[00:45:54] ....................................................................................................
[00:45:57] i...................................................................i...............................
[00:45:57] i...................................................................i...............................
[00:46:00] .......................................................F............................................
[00:46:06] ............................................i.......................................................
[00:46:08] ....................................................................................................
[00:46:12] ....................................................................................................
[00:46:15] ....................................................................................................
[00:46:15] ....................................................................................................
[00:46:18] ....................................................................................................
[00:46:21] ....................................................................................................
[00:46:24] .......F............................................................................................
[00:46:31] ....................................................................................................
[00:46:33] .....................................i..............................................................
[00:46:37] .......................................................................................i.i..ii......
[00:46:37] .......................................................................................i.i..ii......
[00:46:40] ....................................F...............................................................
[00:46:46] ....................................................................................................
[00:46:49] ....................................................................................................
[00:46:52] ....................................................................................................
[00:46:55] ....................................................................................................
[00:46:55] ....................................................................................................
[00:46:58] ..............i.....................................................................................
[00:47:01] ....................................................................................................
[00:47:04] ....................................................................................................
[00:47:08] ...............................................i................................................FF..
[00:47:12] ....................................................................................................
[00:47:14] ....................................................................................................
[00:47:17] .................F........................................................................i.........
[00:47:18] failures:
[00:47:18] 
[00:47:18] ---- [ui] ui/existential_types/no_inferrable_concrete_type.rs stdout ----
[00:47:18] diff of stderr:
[00:47:18] diff of stderr:
[00:47:18] 
[00:47:18] - error[E0391]: cycle detected when normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: Foo }`
[00:47:18] + error[E0283]: type annotations required: cannot resolve `_: std::marker::Copy`
[00:47:18] 2   --> $DIR/no_inferrable_concrete_type.rs:16:1
[00:47:18] 3    |
[00:47:18] 4 LL | existential type Foo: Copy; //~ cycle detected
[00:47:18] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:18] 6    |
[00:47:18] 6    |
[00:47:18] -    = note: ...which again requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: Foo }`, completing the cycle
[00:47:18] +    = note: the return type of a function must have a statically known size
[00:47:18] - error: aborting due to previous error
[00:47:18] + error: defining existential type use differs from previous
[00:47:18] +   --> $DIR/no_inferrable_concrete_type.rs:21:1
[00:47:18] +    |
[00:47:18] +    |
[00:47:18] + LL | / fn main() {
[00:47:18] + LL | |     let _: Foo = std::mem::transmute(0u8);
[00:47:18] + LL | | }
[00:47:18] +    |
[00:47:18] + note: previous use here
[00:47:18] +   --> $DIR/no_inferrable_concrete_type.rs:19:1
[00:47:18] +    |
[00:47:18] +    |
[00:47:18] + LL | fn bar(x: Foo) -> Foo { x }
[00:47:18] 10 
[00:47:18] - For more information about this error, try `rustc --explain E0391`.
[00:47:18] + error: aborting due to 2 previous errors
[00:47:18] + 
[00:47:18] + 
[00:47:18] + For more information about this error, try `rustc --explain E0283`.
[00:47:18] 12 
[00:47:18] 
[00:47:18] 
[00:47:18] The actual stderr differed from the expected stderr.
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/no_inferrable_concrete_type.stderr
[00:47:18] To update references, rerun the tests and pass the `--bless` flag
[00:47:18] To only update this specific test, also pass `--test-args existential_types/no_inferrable_concrete_type.rs`
[00:47:18] error: 1 errors occurred comparing output.
[00:47:18] status: exit code: 1
[00:47:18] status: exit code: 1
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/a" "-Crpath" "-O" "-Z9,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"fn bar(x: Foo) -> Foo { x }","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: defining existential type use differs from previous\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:21:1\n   |\nLL | / fn main() {\nLL | |     let _: Foo = std::mem::transmute(0u8);\nLL | | }\n   | |_^\n   |\nnote: previous use here\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:19:1\n   |\nLL | fn bar(x: Foo) -> Foo { x }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:47:18] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:18] {"message":"For more information about this error, try `rustc --explain E0283`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0283`.\n"}
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] thread '[ui] ui/existential_types/no_inferrable_concrete_type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:18] 
[00:47:18] ---- [ui] ui/impl-trait/where-allowed.rs stdout ----
[00:47:18] diff of stderr:
[00:47:18] 
[00:47:18] 16    |                                                 |       nested `impl Trait` here
[00:47:18] 17    |                                                 outer `impl Trait`
[00:47:18] 18 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 20   --> $DIR/where-allowed.rs:28:40
[00:47:18] 21    |
[00:47:18] 22 LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
[00:47:18] 23    |                                        ^^^^^^^^^^
[00:47:18] 24 
[00:47:18] 24 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 26   --> $DIR/where-allowed.rs:32:42
[00:47:18] 27    |
[00:47:18] 28 LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
[00:47:18] 29    |                                          ^^^^^^^^^^
[00:47:18] 30 
[00:47:18] 30 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 32   --> $DIR/where-allowed.rs:36:38
[00:47:18] 33    |
[00:47:18] 34 LL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }
[00:47:18] 35    |                                      ^^^^^^^^^^
[00:47:18] 36 
[00:47:18] 36 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowturn() -> &'static dyn Fn(impl Debug) { panic!() }
[00:47:18] 59    |                                                       ^^^^^^^^^^
[00:47:18] 60 
[00:47:18] 60 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 62   --> $DIR/where-allowed.rs:56:57
[00:47:18] 63    |
[00:47:18] 64 LL | fn in_dyn_Fn_return_in_return() -> &'static dyn Fn() -> impl Debug { panic!() }
[00:47:18] 65    |                                                         ^^^^^^^^^^
[00:47:18] 66 
[00:47:18] 66 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 68   --> $DIR/where-allowed.rs:60:51
[00:47:18] 69    |
[00:47:18] 70 LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
[00:47:18] 71    |                                                   ^^^^^^^^^^
[00:47:18] 72 
[00:47:18] 72 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 74   --> $DIR/where-allowed.rs:65:53
[00:47:18] 75    |
[00:47:18] 76 LL | fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }
[00:47:18] 77    |                                                     ^^^^^[00:47:18] 119    |                           ^^^^^^^^^^
[00:47:18] 120 
[00:47:18] 120 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 122   --> $DIR/where-allowed.rs:108:25
[00:47:18] 123    |
[00:47:18] 124 LL |     InBraceVariant { x: impl Debug },
[00:47:18] 125    |                         ^^^^^^^^^^
[00:47:18] 126 
[00:47:18] 126 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 128   --> $DIR/where-allowed.rs:110:20
[00:47:18] 129    |
[00:47:18] 130 LL |     InTupleVariant(impl Debug),
[00:47:18] 131    |                    ^^^^^^^^^^
[00:47:18] 132 
[00:47:18] 132 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 134   --> $DIR/where-allowed.rs:121:23
[00:47:18] 135    |
[00:47:18] 136 LL |     fn in_return() -> impl Debug;
[00:47:18] 137    |                       ^^^^^^^^^^
[00:47:18] 138 
[00:47:18] 138 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 140   --> $DIR/where-allowed.rs:132:1impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 164   --> $DIR/where-allowed.rs:167:23
[00:47:18] 165    |
[00:47:18] 166 LL | type InTypeAlias<R> = impl Debug;
[00:47:18] 167    |                       ^^^^^^^^^^
[00:47:18] 168 
[00:47:18] 168 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 170   --> $DIR/where-allowed.rs:170:39
[00:47:18] 171    |
[00:47:18] 172 LL | type InReturnInTypeAlias<R> = fn() -> impl Debug;
[00:47:18] 173    |                                       ^^^^^^^^^^
[00:47:18] 174 
[00:47:18] 174 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 176   --> $DIR/where-allowed.rs:174:16
[00:47:18] 177    |
[00:47:18] 178 LL | impl PartialEq<impl Debug> for () {
[00:47:18] 179    |                ^^^^^^^^^^
[00:47:18] 180 
[00:47:18] 180 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 182   --> $DIR/where-allowed.rs:179:24
[00:47:18] 183    |
[00:47:18] 184 LL | impl PartialEq<()> for impl Debug {
[00:47:18] 185    |                        ^^^^^^^^^^
[00:47:18] 186 
[00:47:18] 186 
[00:47:18] - erroit` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 236   --> $DIR/where-allowed.rs:232:46
[00:47:18] 237    |
[00:47:18] 238 LL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };
[00:47:18] 239    |                                              ^^^^^^^^^
[00:47:18] 240 
[00:47:18] - error: aborting due to 39 previous errors
[00:47:18] + error: aborting due to 38 previous errors
[00:47:18] + error: aborting due to 38 previous errors
[00:47:18] 242 
[00:47:18] 243 Some errors occurred: E0562, E0666.
[00:47:18] 244 For more information about an error, try `rustc --explain E0562`.
[00:47:18] 
[00:47:18] 
[00:47:18] The actual stderr differed from the expected stderr.
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/where-allowed.stderr
[00:47:18] To update references, rerun the tests and pass the `--bless` flag
[00:47:18] To only update this specific test, also pass `--test-args impl-trait/where-allowed.rs`
[00:47:18] error: 1 errors occurred comparing output.
[00:47:18] status: exit code: 1
[00:47:18] status: exit code: 1
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/where-allowed.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/auxiliary" "-A" "unused"
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] ------------------------------------------
[00:47:18] stderr:
[00:47:18] stderr:
[00:47:18] ------------------------------------------
[00:47:18] {"message":"nested `impl Trait` is not allowed","code":{"code":"E0666","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2268,"byte_end":2287,"line_start":60,"line_end":60,"column_start":43,"column_end":62,"is_primary":false,"text":[{"text":"fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }","highlight_start":43,"highlight_end":62}],"label":"outer `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2276,"byte_end":2286,"line_start":60,"line_end":60,"column_start":51,"column_end":61,"is_primary":true,"text":[{"text":"fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }","highlight_start":51,"highlight_end":61}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0666]: nested `impl Trait` is not allowed\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:60:51\n   |\nLL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }\n   |                                           --------^^^^^^^^^^-\n   |                                                            outer `impl Trait`\n\n"}
[00:47:18] {"message":"`impl Trait` not allowed outside of function and inherent method return types or bindings","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types or binding types.\n\nErroneous code example:\n\n