plain
[00:44:58] ......i.............................................................................................
[00:45:02] ............ii.iii..................................................................................
[00:45:04] ....................................................................................................
[00:45:06] ....................................................................................................
[00:45:09] ......................................................................................F.............
[00:45:13] ....................................................................................................
[00:45:16] i...................................................................i...............................
[00:45:16] i...................................................................i...............................
[00:45:19] .......................................................F............................................
[00:45:24] ............................................i.......................................................
[00:45:27] ....................................................................................................
[00:45:31] ....................................................................................................
[00:45:33] ....................................................................................................
[00:45:33] ....................................................................................................
[00:45:36] ....................................................................................................
[00:45:39] ....................................................................................................
[00:45:42] .......F............................................................................................
[00:45:49] ....................................................................................................
[00:45:51] .....................................i..............................................................
[00:45:54] .......................................................................................i.i..ii......
[00:45:54] .......................................................................................i.i..ii......
[00:45:58] ....................................F...............................................................
[00:46:04] ....................................................................................................
[00:46:06] ....................................................................................................
[00:46:09] ....................................................................................................
[00:46:12] ....................................................................................................
[00:46:12] ....................................................................................................
[00:46:15] ..............i.....................................................................................
[00:46:18] ....................................................................................................
[00:46:21] ....................................................................................................
[00:46:25] ...............................................i................................................FF..
[00:46:28] ....................................................................................................
:34] +   --> $DIR/no_inferrable_concrete_type.rs:21:1
[00:46:34] +    |
[00:46:34] + LL | / fn main() {
[00:46:34] + LL | |     let _: Foo = std::mem::transmute(0u8);
[00:46:34] + LL | | }
[00:46:34] +    |
[00:46:34] + note: previous use here
[00:46:34] +   --> $DIR/no_inferrable_concrete_type.rs:19:1
[00:46:34] +    |
[00:46:34] +    |
[00:46:34] + LL | fn bar(x: Foo) -> Foo { x }
[00:46:34] 10 
[00:46:34] - For more information about this error, try `rustc --explain E0391`.
[00:46:34] + error: aborting due to 2 previous errors
[00:46:34] + 
[00:46:34] + 
[00:46:34] + For more information about this error, try `rustc --explain E0283`.
[00:46:34] 12 
[00:46:34] 
[00:46:34] 
[00:46:34] The actual stderr differed from the expected stderr.
[00:46:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/no_inferrable_concrete_type.stderr
[00:46:34] To update references, rerun the tests and pass the `--bless` flag
[00:46:34] To only update this specific test, also pass `--test-args existential_types/no_inferrable_concrete_type.rs`
[00:46:34] error: 1 errors occurred comparing output.
[00:46:34] status: exit code: 1
[00:46:34] status: exit code: 1
[00:46:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/a" "-Crpath" "-O" "-Zbyte_end":725,"line_start":16,"line_end":16,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"existential type Foo: Copy; //~ cycle detected","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the return type of a function must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0283]: type annotations required: cannot resolve `_: std::marker::Copy`\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:16:1\n   |\nLL | existential type Foo: Copy; //~ cycle detected\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the return type of a function must have a statically known size\n\n"}
[00:46:34] {"message":"defining existential type use differs from previous","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs","byte_start":816,"byte_end":872,"line_start":21,"line_end":23,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn main() {","highlight_start":1,"highlight_end":12},{"text":"    let _: Foo = std::mem::transmute(0u8);","highlight_start":1,"highlight_end":43},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous use here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs","byte_start":787,"byte_end":814,"line_start":19,"line_end":1                                 outer `impl Trait`
[00:46:34] 18 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 20   --> $DIR/where-allowed.rs:28:40
[00:46:34] 21    |
[00:46:34] 22 LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
[00:46:34] 23    |                                        ^^^^^^^^^^
[00:46:34] 24 
[00:46:34] 24 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 26   --> $DIR/where-allowed.rs:32:42
[00:46:34] 27    |
[00:46:34] 28 LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
[00:46:34] 29    |                                          ^^^^^^^^^^
[00:46:34] 30 
[00:46:34] 30 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 32   --> $DIR/where-allowed.rs:36:38
[00:46:34] 33    |
[00:46:34] 34 LL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }
[00:46:34] 35    |                                      ^^^^^^^^^^
[00:46:34] 36 
[00:46:34] 36 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allow46:34] 210 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 212   --> $DIR/where-allowed.rs:210:24
[00:46:34] 213    |
[00:46:34] 214 LL |     where T: PartialEq<impl Debug>
[00:46:34] 215    |                        ^^^^^^^^^^
[00:46:34] 216 
[00:46:34] 216 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 218   --> $DIR/where-allowed.rs:217:17
[00:46:34] 219    |
[00:46:34] 220 LL |     where T: Fn(impl Debug)
[00:46:34] 221    |                 ^^^^^^^^^^
[00:46:34] 222 
[00:46:34] 222 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 224   --> $DIR/where-allowed.rs:224:22
[00:46:34] 225    |
[00:46:34] 226 LL |     where T: Fn() -> impl Debug
[00:46:34] 227    |                      ^^^^^^^^^^
[00:46:34] 228 
[00:46:34] 228 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] -   --> $DIR/where-allowed.rs:230:29
[00:46:34] -    |
[00:46:34] - LL |     let _in_local_variable: impl Fn() = || {};
[00:46:34] - 
[00:46:34] - 
[00:46:34] - error[E0562]: `impl Tra                      outer `impl Trait`\n\n"}
[00:46:34] {"message":"`impl Trait` not allowed outside of function and inherent method return types or bindings","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types or binding types.\n\nErroneous code example:\n\n