\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs","byte_start":196,"byte_end":197,"line_start":8,"line_end":8,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    S::<0>; //~ ERROR  wrong number of const arguments","highlight_start":9,"highlight_end":10}],"label":"unexpected const argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of const arguments: expected 0, found 1\n  --> /checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs:8:9\n   |\nLL |     S::<0>; //~ ERROR  wrong number of const arguments\n   |         ^ unexpected const argument\n\n"}
[00:59:57] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:59:57] {"message":"Some errors occurred: E0107, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0107, E0599.\n"}
[00:59:57] 
[00:59:57] ------------------------------------------
[00:59:57] 
[00:59:57] thread '[ui] ui/const-generics/invalid-const-arg-for-type-param.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[00:59:57] thread '[ui] ui/const-generics/invalid-const-arg-for-type-param.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[00:59:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:57] 
[00:59:57] ---- [ui] ui/const-generics/cannot-infer-type-for-const-param.rs stdout ----
[00:59:57] 
[00:59:57] error: Error: expected failure status (Some(1)) but received status None.
[00:59:57] status: signal: 4
[00:59:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/cannot-infer-type-for-const-param/auxiliary" "-A" "unused"
[00:59:57] ------------------------------------------
[00:59:57] 
[00:59:57] ------------------------------------------
[00:59:57] stderr:
[00:59:57] stderr:
[00:59:57] ------------------------------------------
[00:59:57] {"message":"the feature `const_generics` is incomplete and may cause the compiler to crash","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs","byte_start":11,"byte_end":25,"line_start":1,"line_end":1,"column_start":12,"column_end":26,"is_primary":true,"text":[{"text":"#![feature(const_generics)]","highlight_start":12,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: the feature `const_generics` is incomplete and may cause the compiler to crash\n  --> /checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs:1:12\n   |\nLL | #![feature(const_generics)]\n   |            ^^^^^^^^^^^^^^\n\n"}
[00:59:57] thread 'rustc' panicked at 'not yet implemented', src/librustc/ty/relate.rs:707:17
[00:59:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:57] {"message":"unexpected const parent path def Ctor(DefId(0/1:9 ~ cannot_infer_type_for_const_param[317d]::Foo[0]::{{constructor}}[0]), Struct, Fn)","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/ui/const-generics/cannot-infer-type-for-const-param.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![feature(const_generics)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unexpected const parent path def Ctor(DefId(0/1:9 ~ cannot_infer_type_for_const_param[317d]::Foo[0]::{{constructor}}[0]), Struct, Fn)\n\n"}
[00:59:57] thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:354:17
[00:59:57] stack backtrace:
[00:59:57]    0:     0x7f5840b84003 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h6534c233ca93623b
[00:59:57]                                at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
[00:59:57]    1:     0x7f5840b7bcdb - std::sys_common::backtrace::_print::h127e98e74bbf84b1
[00:59:57]                                at src/libstd/sys_common/backtrace.rs:71
[00:59:57]    2:     0x7f5840b80266 - std::panicking::default_hook::{{closure}}::h9f2f0a13f91ccee1
[00:59:57]                                at src/libstd/sys_common/backtrace.rs:59
[00:59:57]                                at src/libstd/panicking.rs:197
[00:59:57]    3:     0x7f5840b7fff9 - std::panicking::default_hook::hdbeb7142e1c5d073
[00:59:57]                                at src/libstd/panicking.rs:211
[00:59:57]    4:     0x7f583e794ef0 - rustc::util::common::panic_hook::h535d18f805e65720
[00:59:57]    5:     0x7f5840b80a58 - std::panicking::rust_panic_with_hook::ha2b8d09f46a27277
[00:59:57]                                at src/libstd/panicking.rs:478
[00:59:57]    6:     0x7f583d3a5274 - std::panicking::begin_panic::hd495adf6beba5efd
[00:59:57]    7:     0x7f583d3bce44 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::he75fa11a3a53cd47
[00:59:57]    8:     0x7f5840e9eb11 - core::ptr::real_drop_in_place::h58a2cb47f278e9ad
[00:59:57]    9:     0x7f5840ea71f3 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h8d154aa15991c3a4
[00:59:57]   10:     0x7f5840e570fb - core::ptr::real_drop_in_place::h3e1ba22969d7e435
[00:59:57]   11:     0x7f5840e54448 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h82ad489bcd1ccdb6
[00:59:57]   12:     0x7f5840e35ee3 - std::thread::local::LocalKey<T>::with::hf1e38a84c0007964
[00:59:57]   13:     0x7f5840e99e64 - scoped_tls::ScopedKey<T>::set::h19abc942033e0ec1
[00:59:57]   14:     0x7f5840ecab6f - syntax::with_globals::hc702a66bb1967a40
[00:59:57]   15:     0x7f5840e55137 - std::sys_common::backtrace::__rust_begin_short_backtrace::hada489d07fa91000
[00:59:57]   16:     0x7f5840b915b9 - __rust_maybe_catch_panic
[00:59:57]                                at src/libpanic_unwind/lib.rs:87
[00:59:57]   17:     0x7f5840e55bf8 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h135a68b86be043b0
[00:59:57]   18:     0x7f5840b62fbe - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hea0fbfb5709bbc5c
[00:59:57]                                at /rustc/56090fedf5ada20c82c01f6267e5390abcddd61f/src/liballoc/boxed.rs:702
[00:59:57]   19:     0x7f5840b9031f - std::sys::unix::thread::Thread::new::thread_start::h5932fc92cd4e83b7
[00:59:57]                                at /rustc/56090fedf5ada20c82c01f6267e5390abcddd61f/src/liballoc/boxed.rs:702
[00:59:57]                                at src/libstd/sys_common/thread.rs:14
[00:59:57]                                at src/libstd/sys/unix/thread.rs:80
[00:59:57]   20:     0x7f583c6a16b9 - start_thread
[00:59:57]   21:     0x7f584085541c - clone
[00:59:57]   22:                0x0 - <unknown>
[00:59:57] thread panicked while panicking. aborting.
[00:59:57] ------------------------------------------
[00:59:57] 
[00:59:57] thread '[ui] ui/const-generics/cannot-infer-type-for-const-param.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[00:59:57] 
---
[00:59:57] 
[00:59:57] 
[00:59:57] The actual stderr differed from the expected stderr.
[00:59:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/issue-60283.stderr
[00:59:57] To update references, rerun the tests and pass the `--bless` flag
[00:59:57] To only update this specific test, also pass `--test-args issues/issue-60283.rs`
[00:59:57] error: 1 errors occurred comparing output.
[00:59:57] status: exit code: 1
[00:59:57] status: exit code: 1
[00:59:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60283.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/auxiliary" "-A" "unused"
[00:59:57] ------------------------------------------
[00:59:57] 
[00:59:57] ------------------------------------------
[00:59:57] stderr:
[00:59:57] stderr:
[00:59:57] ------------------------------------------
[00:59:57] {"message":"type mismatch in function arguments","code":{"code":"E0631","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"found signature of `fn(_) -> _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":91,"byte_end":197,"line_start":9,"line_end":11,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"pub fn foo<T, F>(_: T, _: F)","highlight_start":1,"highlight_end":29},{"text":"where T: for<'a> Trait<'a>,","highlight_start":1,"highlight_end":28},{"text":"      F: for<'a> FnMut(<T as Trait<'a>>::Item) {}","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0631]: type mismatch in function arguments\n  --> /checkout/src/test/ui/issues/issue-60283.rs:14:5\n   |\nLL |     foo((), drop)\n   |     ^^^\n   |     |\n   |     expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`\n   |     found signature of `fn(_) -> _`\n   |\nnote: required by `foo`\n  --> /checkout/src/test/ui/issues/issue-60283.rs:9:1\n   |\nLL | / pub fn foo<T, F>(_: T, _: F)\nLL | | where T: for<'a> Trait<'a>,\nLL | |       F: for<'a> FnMut(<T as Trait<'a>>::Item) {}\n   | |_________________________________________________^\n\n"}
[00:59:57] {"message":"type mismatch resolving `for<'a> <fn(_) {std::mem::drop::<_>} as std::ops::FnOnce<(<() as Trait<'a>>::Item,)>>::Output == ()`","code":{"code":"E0271","explanation":"\nThis is because of a type mismatch between the associated type of some\ntrait (e.g., `T::Bar`, where `T` implements `trait Quux { type Bar; }`)\nand another type `U` that is required to be equal to `T::Bar`, but is not.\nExamples follow.\n\nHere is a basic example:\n\n