plain
[00:44:47] ......i.............................................................................................
[00:44:50] ............ii.iii..................................................................................
[00:44:52] ....................................................................................................
[00:44:54] ....................................................................................................
[00:44:57] ......................................................................................F.............
[00:45:01] ....................................................................................................
[00:45:04] i...................................................................i...............................
[00:45:04] i...................................................................i...............................
[00:45:07] .......................................................F............................................
[00:45:12] ............................................i.......................................................
[00:45:15] ....................................................................................................
[00:45:18] ....................................................................................................
[00:45:20] ....................................................................................................
[00:45:20] ....................................................................................................
[00:45:23] ....................................................................................................
[00:45:26] ....................................................................................................
[00:45:29] .......F............................................................................................
[00:45:35] ....................................................................................................
[00:45:38] .....................................i..............................................................
[00:45:41] .......................................................................................i.i..ii......
[00:45:41] .......................................................................................i.i..ii......
[00:45:44] ....................................F...............................................................
[00:45:50] ....................................................................................................
[00:45:53] ....................................................................................................
[00:45:55] ....................................................................................................
[00:45:58] ....................................................................................................
[00:45:58] ....................................................................................................
[00:46:02] ..............i.....................................................................................
[00:46:05] ....................................................................................................
[00:46:07] ....................................................................................................
[00:46:11] ...............................................i................................................FF..
[00:46:14] ....................................................................................................
[00:46:17] ....................................................................................................
[00:46:19] .................F........................................................................i.........
9,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"fn bar(x: Foo) -> Foo { x }","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: defining existential type use differs from previous\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:21:1\n   |\nLL | / fn main() {\nLL | |     let _: Foo = std::mem::transmute(0u8);\nLL | | }\n   | |_^\n   |\nnote: previous use here\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:19:1\n   |\nLL | fn bar(x: Foo) -> Foo { x }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:46:20] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:20] {"message":"For more information about this error, try `rustc --explain E0283`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0283`.\n"}
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] thread '[ui] ui/existential_types/no_inferrable_concrete_type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:20] 
[00:46:20] ---- [ui] ui/impl-trait/where-allowed.rs stdout ----
[00:46:20] diff of stderr:
[00:46:20] 
[00:46:20] 16    |                                                 |       nested `impl Trait` here
[00:46:20] 78 
[00:46:20] 78 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 80   --> $DIR/where-allowed.rs:69:57
[00:46:20] 81    |
[00:46:20] 82 LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
[00:46:20] 83    |                                                         ^^^^^^^^^^
[00:46:20] 84 
[00:46:20] 84 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 86   --> $DIR/where-allowed.rs:74:59
[00:46:20] 87    |
[00:46:20] 88 LL | fn in_impl_Fn_return_in_return() -> &'static impl Fn() -> impl Debug { panic!() }
[00:46:20] 89    |                                                           ^^^^^^^^^^
[00:46:20] 90 
[00:46:20] 90 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 92   --> $DIR/where-allowed.rs:78:38
[00:46:20] 93    |
[00:46:20] 94 LL | fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }
[00:46:20] 95    |                                      ^^^^^^^^^^
[00:46:20] 96 
[00:46:20] 96 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0[00:46:20] 119    |                           ^^^^^^^^^^
[00:46:20] 120 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 122   --> $DIR/where-allowed.rs:108:25
[00:46:20] 123    |
[00:46:20] 124 LL |     InBraceVariant { x: impl Debug },
[00:46:20] 125    |                         ^^^^^^^^^^
[00:46:20] 126 
[00:46:20] 126 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 128   --> $DIR/where-allowed.rs:110:20
[00:46:20] 129    |
[00:46:20] 130 LL |     InTupleVariant(impl Debug),
[00:46:20] 131    |                    ^^^^^^^^^^
[00:46:20] 132 
[00:46:20] 132 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 134   --> $DIR/where-allowed.rs:121:23
[00:46:20] 135    |
[00:46:20] 136 LL |     fn in_return() -> impl Debug;
[00:46:20] 137    |                       ^^^^^^^^^^
[00:46:20] 138 
[00:46:20] 138 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 140   --> $DIR/where-allowed.rs:132:16
[00:46:20] 141    |
[00:46:20] 142 LL |     type Out = impl Debug;
[00:46:20] 143    |                ^^^^^^^^^^
[00:46:20] 144 
[00:46:20] 144 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 146   --> $DIR/where-allowed.rs:138:34
[00:46:20] 147    |
[00:46:20] 148 LL |     fn in_trait_impl_return() -> impl Debug { () }
[00:46:20] 149    |                                  ^^^^^^^^^^
[00:46:20] 150 
[00:46:20] 150 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 152   --> $DIR/where-allowed.rs:151:33
[00:46:20] 153    |
[00:46:20] 154 LL |     fn in_foreign_parameters(_: impl Debug);
[00:46:20] 155    |                                 ^^^^^^^^^^
[00:46:20] 156 
[00:46:20] 156 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 158   --> $DIR/where-allowed.rs:154:31
[00:46:20] 159    |
[00:46:20] 160 LL |     fn in_foreign_return() -> impl Debug;
[00:46:20] 161    |                               ^^^^^^^^^^
[00:46:20] 162 
[00:46:20] 162 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `46:20] 210 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 212   --> $DIR/where-allowed.rs:210:24
[00:46:20] 213    |
[00:46:20] 214 LL |     where T: PartialEq<impl Debug>
[00:46:20] 215    |                        ^^^^^^^^^^
[00:46:20] 216 
[00:46:20] 216 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 218   --> $DIR/where-allowed.rs:217:17
[00:46:20] 219    |
[00:46:20] 220 LL |     where T: Fn(impl Debug)
[00:46:20] 221    |                 ^^^^^^^^^^
[00:46:20] 222 
[00:46:20] 222 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:20] 224   --> $DIR/where-allowed.rs:224:22
[00:46:20] 225    |
[00:46:20] 226 LL |     where T: Fn() -> impl Debug
[00:46:20] 227    |                      ^^^^^^^^^^
[00:46:20] 228 
[00:46:20] 228 
[00:46:20] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:20] -   --> $DIR/where-allowed.rs:230:29
[00:46:20] -    |
[00:46:20] - LL |     let _in_local_variable: impl Fn() = || {};
[00:46:20] - 
[00:46:20] - 
[00:46:20] - error[E0562]: `impl Tra     |       |\n   |                                           |       nested `impl Trait` here\n   |                                           outer `impl Trait`\n\n"}
[00:46:20] {"message":"nested `impl Trait` is not allowed","code":{"code":"E0666","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2682,"byte_end":2701,"line_start":69,"line_end":69,"column_start":49,"column_end":68,"is_primary":false,"text":[{"text":"fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }","highlight_start":49,"highlight_end":68}],"label":"outer `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2690,"byte_end":2700,"line_start":69,"line_end":69,"column_start":57,"column_end":67,"is_primary":true,"text":[{"text":"fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }","highlight_start":57,"highlight_end":67}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0666]: nested `impl Trait` is not allowed\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:69:57\n   |\nLL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }\n   |                                                 --------^^^^^^^^^^-\n   |                                                 |       |\n   |                                                 |       nested `impl Trait` here\n   |                           ment":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:36:38\n   |\nLL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }\n   |                                      ^^^^^^^^^^\n\n"}
[00:46:20] {"message":"`impl Trait` not allowed outside of function and inherent method return types or bindings","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types or binding types.\n\nErroneous code example:\n\n