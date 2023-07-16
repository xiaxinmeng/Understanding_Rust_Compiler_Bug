plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/18/61/4e0f977cfe063188d73622a91cab8b8b409b662f422303fc687f362d941f/awscli-1.15.18-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 15.1MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:19:38] travis_time:end:stage0-rustc:start=1525986470887947580,finish=1525987357605724083,duration=886717776503

[00:19:38] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:19:39]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:19:39]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:19:39]    Compiling num_cpus v1.8.0
[00:19:42]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:20:42]     Finished release [optimized] target(s) in 62.60 secs
[00:20:42] travis_fold:start:stage0-rustc_codegen_llvm
---
[00:32:10] travis_time:end:stage1-rustc:start=1525987506379069200,finish=1525988109530539708,duration=603151470508

[00:32:10] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:32:11]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:32:11]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:32:11]    Compiling num_cpus v1.8.0
[00:32:12]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:32:57]     Finished release [optimized] target(s) in 46.27 secs
[00:32:57] travis_fold:start:stage1-rustc_codegen_llvm
---
[00:51:10] ............................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:51:30] ........................................................................
[00:51:50] ...................................................................................ii...............
[00:52:40] ...............................................i....................................................
[00:52:51] i.ii............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:53:39] ........iiiiiii.....................................................................................
[00:53:59] ....................................................................................................
[00:54:18] ....................................................................................................
[00:54:36] ...............................................................................
---
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:18] 
[00:58:18] running 39 tests
[00:58:21] iFFFFFFFiFFFFFFFFFFFF.F.FFFFFFFiFFFFFFF
[00:58:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/function-as-argument.rs stdout ----
[00:58:21]  
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(char, f64), (char, f64)>
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(u32, &str), (u32, &str)>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::function[0]<char, f64>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::function[0]<f32, i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::function[0]<i32, ()>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::function[0]<u32, &str>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::take_fn_once[0]<char, f64, fn(char, f64)>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::take_fn_once[0]<u32, &str, fn(u32, &str)>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::take_fn_pointer[0]<f32, i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn function_as_argument::take_fn_pointer[0]<i32, ()>
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/function-as-argument.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:21] 
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]; 2]> @@ drop_in_place_intrinsic0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]]> @@ drop_in_place_intrinsic0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<drop_in_place_intrinsic::StructWithDtor[0]> @@ drop_in_place_intrinsic0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn drop_in_place_intrinsic::drop_slice_in_place[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn drop_in_place_intrinsic::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn drop_in_place_intrinsic::{{impl}}[0]::drop[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/cross-crate-trait-method.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/cross-crate-trait-method.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::Trait[0]::with_default_impl[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::Trait[0]::with_default_impl[0]<u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::Trait[0]::with_default_impl_generic[0]<char, i16>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::Trait[0]::with_default_impl_generic[0]<char, i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::Trait[0]::with_default_impl_generic[0]<u32, &str>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::Trait[0]::with_default_impl_generic[0]<u32, bool>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::{{impl}}[0]::without_default_impl_generic[0]<bool>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::{{impl}}[0]::without_default_impl_generic[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::{{impl}}[1]::without_default_impl_generic[0]<bool>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_export_trait_method::{{impl}}[1]::without_default_impl_generic[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn cross_crate_trait_method::start[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/cross-crate-trait-method.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/cross-crate-generic-functions.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/cross-crate-generic-functions.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_generic_function::bar[0]<u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_generic_function::bar[0]<u64>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_generic_function::foo[0]<u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_generic_function::foo[0]<u64>
[00:58:21] TRANS_ITEM MONO_ITEM fn cross_crate_generic_functions::start[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/cross-crate-generic-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/generic-functions.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/generic-functions.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo1[0]<&str>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo1[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo1[0]<i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo1[0]<i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo2[0]<&str, usize>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo2[0]<char, ()>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo2[0]<i32, i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo2[0]<i64, &str>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo3[0]<char, (), ()>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo3[0]<i16, &str, usize>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo3[0]<i32, i32, i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::foo3[0]<i64, &str, char>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::lifetime_only[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_functions::start[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/generic-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<f64, f32>> @@ generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<i32, i64>> @@ generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::NonGenericWithDrop[0]> @@ generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructNoDrop[0]<generic_drop_glue::NonGenericWithDrop[0], f64>> @@ generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<&str, generic_drop_glue::NonGenericNoDrop[0]>> @@ generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<i8, char>> @@ generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_drop_glue::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_drop_glue::{{impl}}[0]::drop[0]<&str, generic_drop_glue::NonGenericNoDrop[0]>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_drop_glue::{{impl}}[0]::drop[0]<i8, char>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_drop_glue::{{impl}}[1]::drop[0]<f64, f32>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_drop_glue::{{impl}}[1]::drop[0]<i32, i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_drop_glue::{{impl}}[2]::drop[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/generic-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/impl-in-non-instantiated-generic.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/impl-in-non-instantiated-generic.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn impl_in_non_instantiated_generic::generic_function[0]::{{impl}}[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn impl_in_non_instantiated_generic::start[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/impl-in-non-instantiated-generic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<instantiation_through_vtable::Struct[0]<u32>> @@ instantiation_through_vtable0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<instantiation_through_vtable::Struct[0]<u64>> @@ instantiation_through_vtable0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn instantiation_through_vtable::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn instantiation_through_vtable::{{impl}}[0]::bar[0]<u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn instantiation_through_vtable::{{impl}}[0]::bar[0]<u64>
[00:58:21] TRANS_ITEM MONO_ITEM fn instantiation_through_vtable::{{impl}}[0]::foo[0]<u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn instantiation_through_vtable::{{impl}}[0]::foo[0]<u64>
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/generic-impl.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/generic-impl.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::id[0]<&str>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::id[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::id[0]<generic_impl::Struct[0]<&str>>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::id[0]<i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::id[0]<i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::get[0]<char, i16>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::get[0]<generic_impl::Struct[0]<&str>, i16>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::get[0]<i32, i16>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::get[0]<i64, i16>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::new[0]<&str>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::new[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::new[0]<generic_impl::Struct[0]<&str>>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::new[0]<i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[0]::new[0]<i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[1]::bar[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[1]::baz[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn generic_impl::{{impl}}[1]::foo[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/generic-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/items-within-generic-items.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/items-within-generic-items.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn items_within_generic_items::generic_fn[0]::nested_fn[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn items_within_generic_items::generic_fn[0]::nested_fn[1]
[00:58:21] TRANS_ITEM MONO_ITEM fn items_within_generic_items::generic_fn[0]<i64>
[00:58:21] TRANS_ITEM MONO_ITEM fn items_within_generic_items::generic_fn[0]<i8>
[00:58:21] TRANS_ITEM MONO_ITEM fn items_within_generic_items::generic_fn[0]<u16>
[00:58:21] TRANS_ITEM MONO_ITEM fn items_within_generic_items::start[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/items-within-generic-items.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::EnumWithDrop[0]> @@ non_generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::StructWithDrop[0]> @@ non_generic_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_drop_glue::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_drop_glue::{{impl}}[0]::drop[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_drop_glue::{{impl}}[1]::drop[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/non-generic-functions.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/non-generic-functions.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::bar[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::bar[0]::baz[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::foo[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::foo[0]::foo[1]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::{{impl}}[0]::bar[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::{{impl}}[0]::bar[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::{{impl}}[0]::bar[0]::foo[1]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::{{impl}}[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::{{impl}}[0]::foo[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn non_generic_functions::{{impl}}[0]::foo[0]::foo[1]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/non-generic-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/overloaded-operators.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/overloaded-operators.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn overloaded_operators::{{impl}}[0]::index[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn overloaded_operators::{{impl}}[1]::index_mut[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn overloaded_operators::{{impl}}[2]::add[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn overloaded_operators::{{impl}}[3]::deref[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn overloaded_operators::{{impl}}[4]::eq[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn overloaded_operators::{{impl}}[4]::ne[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/overloaded-operators.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/static-init.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/static-init.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn static_init::foo[0]<i32>
[00:58:21] TRANS_ITEM MONO_ITEM fn static_init::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM static static_init::FN[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/static-init.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/statics-and-consts.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/statics-and-consts.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn statics_and_consts::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn statics_and_consts::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM static statics_and_consts::STATIC1[0]
[00:58:21] TRANS_ITEM MONO_ITEM static statics_and_consts::foo[0]::STATIC2[0]
[00:58:21] TRANS_ITEM MONO_ITEM static statics_and_consts::foo[0]::STATIC2[1]
[00:58:21] TRANS_ITEM MONO_ITEM static statics_and_consts::foo[0]::STATIC2[2]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/statics-and-consts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/trait-implementations.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/trait-implementations.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[1]::bar[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[1]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[2]::bar[0]<&str>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[2]::bar[0]<()>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[2]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[3]::bar[0]<&str, &str>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[3]::bar[0]<u32, ()>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[3]::foo[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_implementations::{{impl}}[3]::foo[0]<i64>
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/trait-implementations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/trait-method-as-argument.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/trait-method-as-argument.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::FnMut[0]::call_mut[0]<fn(char) -> char, (char)>
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::FnMut[0]::call_mut[0]<fn(u32) -> u32, (u32)>
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(char) -> char, (char)>
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::FnOnce[0]::call_once[0]<fn(u32) -> u32, (u32)>
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::Fn[0]::call[0]<fn(char) -> char, (char)>
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ops[0]::function[0]::Fn[0]::call[0]<fn(u32) -> u32, (u32)>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::Trait[0]::foo[0]<char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::take_foo[0]<char, fn(char) -> char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::take_foo[0]<u32, fn(u32) -> u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::take_foo_mut[0]<char, fn(char) -> char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::take_foo_mut[0]<u32, fn(u32) -> u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::take_foo_once[0]<char, fn(char) -> char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::take_foo_once[0]<u32, fn(u32) -> u32>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_as_argument::{{impl}}[0]::foo[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/trait-method-as-argument.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/trait-method-default-impl.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/trait-method-default-impl.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeGenericTrait[0]::bar[0]<i32, u64, &str>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeGenericTrait[0]::bar[0]<i32, u64, char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeGenericTrait[0]::bar[0]<u32, i16, ()>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeGenericTrait[0]::bar[0]<u32, i8, &[char; 1]>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeGenericTrait[0]::foo[0]<i32, u64>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeTrait[0]::bar[0]<i8, &str>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeTrait[0]::bar[0]<i8, char>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::SomeTrait[0]::foo[0]<i8>
[00:58:21] TRANS_ITEM MONO_ITEM fn trait_method_default_impl::start[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/trait-method-default-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<i16>> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<u32>> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Intermediate[0]> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<i16>> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<u32>> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Leaf[0]> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<i16>> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<u32>> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Root[0]> @@ transitive_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn transitive_drop_glue::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn transitive_drop_glue::{{impl}}[0]::drop[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn transitive_drop_glue::{{impl}}[1]::drop[0]<i16>
[00:58:21] TRANS_ITEM MONO_ITEM fn transitive_drop_glue::{{impl}}[1]::drop[0]<u32>
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/transitive-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(i16, (tuple_drop_glue::Dropped[0], bool))> @@ tuple_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(tuple_drop_glue::Dropped[0], bool)> @@ tuple_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, tuple_drop_glue::Dropped[0])> @@ tuple_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<tuple_drop_glue::Dropped[0]> @@ tuple_drop_glue0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn tuple_drop_glue::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn tuple_drop_glue::{{impl}}[0]::drop[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/tuple-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/unused-traits-and-generics.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/unused-traits-and-generics.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn unused_traits_and_generics::{{impl}}[3]::foo[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/unused-traits-and-generics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<bool> @@ unsizing0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<char> @@ unsizing0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<f64> @@ unsizing0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<u32> @@ unsizing0[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn unsizing::start[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn unsizing::{{impl}}[0]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn unsizing::{{impl}}[1]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn unsizing::{{impl}}[2]::foo[0]
[00:58:21] TRANS_ITEM MONO_ITEM fn unsizing::{{impl}}[3]::foo[0]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/item-collection/unsizing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue[Internal] extern_drop_glue-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_drop_glue::mod1[0]::user[0] @@ extern_drop_glue-mod1[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_drop_glue::user[0] @@ extern_drop_glue[External]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])> @@ local_drop_glue-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]> @@ local_drop_glue[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]> @@ local_drop_glue[Internal] local_drop_glue-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]> @@ local_drop_glue-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_drop_glue::mod1[0]::user[0] @@ local_drop_glue-mod1[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_drop_glue::user[0] @@ local_drop_glue[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_drop_glue::{{impl}}[0]::drop[0] @@ local_drop_glue[External]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_explicit_inlining::always_inlined[0] @@ inlining_from_extern_crate[Internal] inlining_from_extern_crate-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_explicit_inlining::inlined[0] @@ inlining_from_extern_crate[Internal] inlining_from_extern_crate-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn inlining_from_extern_crate::mod1[0]::user[0] @@ inlining_from_extern_crate-mod1[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn inlining_from_extern_crate::mod2[0]::user[0] @@ inlining_from_extern_crate-mod2[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn inlining_from_extern_crate::user[0] @@ inlining_from_extern_crate[External]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_generic_function::bar[0]<&str> @@ cgu_generic_function.volatile[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn cgu_generic_function::foo[0]<&str> @@ cgu_generic_function.volatile[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_generic::mod1[0]::mod1[0]::user[0] @@ extern_generic-mod1-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_generic::mod1[0]::user[0] @@ extern_generic-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_generic::mod2[0]::user[0] @@ extern_generic-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_generic::mod3[0]::non_user[0] @@ extern_generic-mod3[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn extern_generic::user[0] @@ extern_generic[Internal]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn local_generic::generic[0]<&str> @@ local_generic.volatile[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn localunits/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
[00:58:21]  
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn local_inlining::inline[0]::inlined_function[0] @@ local_inlining-user1[Internal] local_inlining-user2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_inlining::non_user[0]::baz[0] @@ local_inlining-non_user[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_inlining::user1[0]::foo[0] @@ local_inlining-user1[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_inlining::user2[0]::bar[0] @@ local_inlining-user2[External]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn local_transitive_inlining::direct_user[0]::foo[0] @@ local_transitive_inlining-indirect_user[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_transitive_inlining::indirect_user[0]::bar[0] @@ local_transitive_inlining-indirect_user[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_transitive_inlining::inline[0]::inlined_function[0] @@ local_transitive_inlining-indirect_user[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn local_transitive_inlining::non_user[0]::baz[0] @@ local_transitive_inlining-non_user[External]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::bar[0] @@ regular_modules[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::foo[0] @@ regular_modules[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod1[0]::bar[0] @@ regular_modules-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod1[0]::foo[0] @@ regular_modules-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::bar[0] @@ regular_modules-mod1-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::foo[0] @@ regular_modules-mod1-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::bar[0] @@ regular_modules-mod1-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::foo[0] @@ regular_modules-mod1-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod2[0]::bar[0] @@ regular_modules-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod2[0]::foo[0] @@ regular_modules-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::bar[0] @@ regular_modules-mod2-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::foo[0] @@ regular_modules-mod2-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::bar[0] @@ regular_modules-mod2-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::foo[0] @@ regular_modules-mod2-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::BAZ[0] @@ regular_modules[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::mod1[0]::BAZ[0] @@ regular_modules-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::mod1[0]::mod1[0]::BAZ[0] @@ regular_modules-mod1-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::mod1[0]::mod2[0]::BAZ[0] @@ regular_modules-mod1-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::mod2[0]::BAZ[0] @@ regular_modules-mod2[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::mod2[0]::mod1[0]::BAZ[0] @@ regular_modules-mod2-mod1[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static regular_modules::mod2[0]::mod2[0]::BAZ[0] @@ regular_modules-mod2-mod2[Internal]
[00:58:21] 
[00:58:21] thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2291:13
[00:58:21] 
[00:58:21] ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
[00:58:21] ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
[00:58:21]  
[00:58:21] These items should have been contained but were not:
[00:58:21] 
[00:58:21] TRANS_ITEM MONO_ITEM fn statics::function[0] @@ statics[External]
[00:58:21] TRANS_ITEM MONO_ITEM fn statics::mod1[0]::function[0] @@ statics-mod1[External]
[00:58:21] TRANS_ITEM MONO_ITEM static statics::BAR[0] @@ statics[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static statics::FOO[0] @@ statics[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static statics::function[0]::BAR[0] @@ statics[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static statics::function[0]::FOO[0] @@ statics[Internal]
[00:58:21] TRANS_ITEM MONO_ITEM static statics::mod1[0]::BAR[0] @@ statics-mod1[Internal]
---
[00:58:21] test result: FAILED. 2 passed; 34 failed; 3 ignored; 0 measured; 0 filtered out
[00:58:21] 
[00:58:21] 
[00:58:21] 
[00:58:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflagsThu, 10 May 2018 22:01:20 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
