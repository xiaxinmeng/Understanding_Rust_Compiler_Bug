plain
[00:53:19] ---- [ui] ui/impl-trait/where-allowed.rs stdout ----
[00:53:19] diff of stderr:
[00:53:19] 
[00:53:19] 231    |
[00:53:19] 232 LL |     let _in_local_variable: impl Fn() = || {};
[00:53:19] +    |
[00:53:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:53:19] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:53:19] 234 
[00:53:19] 234 
[00:53:19] 235 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:53:19] 236   --> $DIR/where-allowed.rs:232:46
[00:53:19] 
[00:53:19] The actual stderr differed from the expected stderr.
[00:53:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/where-allowed.stderr
[00:53:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/where-allowed.stderr
[00:53:19] To update references, rerun the tests and pass the `--bless` flag
[00:53:19] To only update this specific test, also pass `--test-args impl-trait/where-allowed.rs`
[00:53:mpl Fn(impl Debug)) { panic!() }","highlight_start":51,"highlight_end":61}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0666]: nested `impl Trait` is not allowed\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:60:51\n   |\nLL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }\n   |                                           --------^^^^^^^^^^-\n   |                                           |       |\n   |                                           |       nested `impl Trait` here\n   |                                           outer `impl Trait`\n\n"}
[00:53:19] {"message":"nested `impl Trait` is not allowed","code":{"code":"E0666","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2682,"byte_end":2701,"line_start":69,"line_end":69,"column_start":49,"column_end":68,"is_primary":false,"text":[{"text":"fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }","highlight_start":49,"highlight_end":68}],"label":"outer `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2690,"byte_end":2700,"line_start":69,"line_end":69,"column_start":57,"column_end":67,"is_primary":true,"text":[{"text":"fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }","highlight_start":57,"highlight_end":67}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestionline_start":28,"line_end":28,"column_start":40,"column_end":50,"is_primary":true,"text":[{"text":"fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }","highlight_start":40,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0562]: `impl Trait` not allowed outside of function and inherent method return types\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:28:40\n   |\nLL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }\n   |                                        ^^^^^^^^^^\n\n"}
[00:53:19] {"message":"`impl Trait` not allowed outside of function and inherent method return types","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types.\n\nErroneous code example:\n\n