\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs","byte_start":196,"byte_end":197,"line_start":8,"line_end":8,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    S::<0>; //~ ERROR  wrong number of const arguments","highlight_start":9,"highlight_end":10}],"label":"unexpected const argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of const arguments: expected 0, found 1\n  --> /checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs:8:9\n   |\nLL |     S::<0>; //~ ERROR  wrong number of const arguments\n   |         ^ unexpected const argument\n\n"}
[01:08:16] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:08:16] {"message":"Some errors occurred: E0107, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0107, E0599.\n"}
[01:08:16] 
[01:08:16] ------------------------------------------
[01:08:16] 
[01:08:16] thread '[ui] ui/const-generics/invalid-const-arg-for-type-param.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:16] thread '[ui] ui/const-generics/invalid-const-arg-for-type-param.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:16] 
[01:08:16] ---- [ui] ui/const-generics/cannot-infer-type-for-const-param.rs stdout ----
[01:08:16] 
[01:08:16] error: Error: expected failure status (Some(1)) but received status None.
[01:08:16] status: signal: 4
[01:08:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/auxiliary" "-A" "unused"
[01:08:16] ------------------------------------------
[01:08:16] 
[01:08:16] ------------------------------------------
[01:08:16] stderr:
[01:08:16] stderr:
[01:08:16] ------------------------------------------
[01:08:16] {"message":"the feature `const_generics` is incomplete and may cause the compiler to crash","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs","byte_start":11,"byte_end":25,"line_start":1,"line_end":1,"column_start":12,"column_end":26,"is_primary":true,"text":[{"text":"#![feature(const_generics)]","highlight_start":12,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: the feature `const_generics` is incomplete and may cause the compiler to crash\n  --> /checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs:1:12\n   |\nLL | #![feature(const_generics)]\n   |            ^^^^^^^^^^^^^^\n\n"}
[01:08:16] thread 'rustc' panicked at 'not yet implemented', src/librustc/ty/relate.rs:707:17
[01:08:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:16] {"message":"unexpected const parent path def Ctor(DefId(0/1:9 ~ cannot_infer_type_for_const_param[317d]::Foo[0]::{{constructor}}[0]), Struct, Fn)","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![feature(const_generics)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unexpected const parent path def Ctor(DefId(0/1:9 ~ cannot_infer_type_for_const_param[317d]::Foo[0]::{{constructor}}[0]), Struct, Fn)\n\n"}
[01:08:16] thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:354:17
[01:08:16] stack backtrace:
[01:08:16]    0:     0x7f5dbe602393 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h9b1566f2fc6c0b14
[01:08:16]    1:     0x7f5dbe5f9688 - std::sys_common::backtrace::_print::hb21fbb9eba3486a2
[01:08:16]    2:     0x7f5dbe5fe0c3 - std::panicking::default_hook::{{closure}}::hb05909e885ff80ac
[01:08:16]    3:     0x7f5dbe5fddd6 - std::panicking::default_hook::h6957af0d428a3ff8
[01:08:16]    4:     0x7f5dbb549a20 - rustc::util::common::panic_hook::h1066cca55dab5d96
[01:08:16]    5:     0x7f5dbe5fe9ad - std::panicking::rust_panic_with_hook::h16eec84e58478d23
[01:08:16]    6:     0x7f5db9efcff7 - std::panicking::begin_panic::hbbc2612b3b0d937e
[01:08:16]    7:     0x7f5db9f24ac8 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::h789c037e32dfa601
[01:08:16]    8:     0x7f5dbe977311 - core::ptr::real_drop_in_place::h8513bf780bdd31fb
[01:08:16]    9:     0x7f5dbe981806 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::hec32f528186f609e
[01:08:16]   10:     0x7f5dbe92f62c - core::ptr::real_drop_in_place::h8123f7814390c64c
[01:08:16]   11:     0x7f5dbe928864 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h55aa09af97447629
[01:08:16]   12:     0x7f5dbe8dfade - std::thread::local::LocalKey<T>::with::h2152a510bf1a65e3
[01:08:16]   13:     0x7f5dbe96ba01 - scoped_tls::ScopedKey<T>::set::h87b1322248244ec4
[01:08:16]   14:     0x7f5dbe9a3aee - syntax::with_globals::h095dba059cc69940
[01:08:16]   15:     0x7f5dbe8e993c - std::sys_common::backtrace::__rust_begin_short_backtrace::hd083b97d703a6a74
[01:08:16]   16:     0x7f5dbe612989 - __rust_maybe_catch_panic
[01:08:16]   17:     0x7f5dbe9215c9 - std::panicking::try::hc16e610a49dbb7c8
[01:08:16]   18:     0x7f5dbe8ec57d - core::ops::function::FnOnce::call_once{{vtable.shim}}::h944d87485e934a02
[01:08:16]   19:     0x7f5dbe5dad1d - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h6ecf1422c2bb3547
[01:08:16]   20:     0x7f5dbe61119f - std::sys::unix::thread::Thread::new::thread_start::h7863e3cf1499fcda
[01:08:16]   21:     0x7f5db91c76b9 - start_thread
[01:08:16]   22:     0x7f5dbe2c241c - clone
[01:08:16]   23:                0x0 - <unknown>
[01:08:16] thread panicked while panicking. aborting.
[01:08:16] ------------------------------------------
[01:08:16] 
[01:08:16] thread '[ui] ui/const-generics/cannot-infer-type-for-const-param.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:08:16] 
---
[01:08:16] 
[01:08:16] 
[01:08:16] The actual stderr differed from the expected stderr.
[01:08:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/issue-60283.stderr
[01:08:16] To update references, rerun the tests and pass the `--bless` flag
[01:08:16] To only update this specific test, also pass `--test-args issues/issue-60283.rs`
[01:08:16] error: 1 errors occurred comparing output.
[01:08:16] status: exit code: 1
[01:08:16] status: exit code: 1
[01:08:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/auxiliary" "-A" "unused"
[01:08:16] ------------------------------------------
[01:08:16] 
[01:08:16] ------------------------------------------
[01:08:16] stderr:
[01:08:16] stderr:
[01:08:16] ------------------------------------------
[01:08:16] {"message":"type mismatch in function arguments","code":{"code":"E0631","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"found signature of `fn(_) -> _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":91,"byte_end":197,"line_start":9,"line_end":11,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"pub fn foo<T, F>(_: T, _: F)","highlight_start":1,"highlight_end":29},{"text":"where T: for<'a> Trait<'a>,","highlight_start":1,"highlight_end":28},{"text":"      F: for<'a> FnMut(<T as Trait<'a>>::Item) {}","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0631]: type mismatch in function arguments\n  --> /checkout/src/test/ui/issues/issue-60283.rs:14:5\n   |\nLL |     foo((), drop)\n   |     ^^^\n   |     |\n   |     expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`\n   |     found signature of `fn(_) -> _`\n   |\nnote: required by `foo`\n  --> /checkout/src/test/ui/issues/issue-60283.rs:9:1\n   |\nLL | / pub fn foo<T, F>(_: T, _: F)\nLL | | where T: for<'a> Trait<'a>,\nLL | |       F: for<'a> FnMut(<T as Trait<'a>>::Item) {}\n   | |_________________________________________________^\n\n"}
[01:08:16] {"message":"type mismatch resolving `for<'a> <fn(_) {std::mem::drop::<_>} as std::ops::FnOnce<(<() as Trait<'a>>::Item,)>>::Output == ()`","code":{"code":"E0271","explanation":"\nThis is because of a type mismatch between the associated type of some\ntrait (e.g., `T::Bar`, where `T` implements `trait Quux { type Bar; }`)\nand another type `U` that is required to be equal to `T::Bar`, but is not.\nExamples follow.\n\nHere is a basic example:\n\n