plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:21] 
[00:50:21] running 39 tests
[00:50:22] iF....F.iF....F......F.F.FFFFFFFiFFFFFF
[00:50:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]; 2]>
[00:50:22]   expected: drop_in_place_intrinsic0[Internal] 
[00:50:22]   actual:   drop_in_place_intrinsic.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]]>
[00:50:22]   expected: drop_in_place_intrinsic0[Internal] 
[00:50:22]   actual:   drop_in_place_intrinsic.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<drop_in_place_intrinsic::StructWithDtor[0]>
[00:50:22]   expected: drop_in_place_intrinsic0[Internal] 
[00:50:22]   actual:   drop_in_place_intrinsic.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----
[00:50:22] ---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<f64, f32>>
[00:50:22]   expected: generic_drop_glue0[Internal] 
[00:50:22]   actual:   generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<i32, i64>>
[00:50:22]   expected: generic_drop_glue0[Internal] 
[00:50:22]   actual:   generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::NonGenericWithDrop[0]>
[00:50:22]   expected: generic_drop_glue0[Internal] 
[00:50:22]   actual:   generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructNoDrop[0]<generic_drop_glue::NonGenericWithDrop[0], f64>>
[00:50:22]   expected: generic_drop_glue0[Internal] 
[00:50:22]   actual:   generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<&str, generic_drop_glue::NonGenericNoDrop[0]>>
[00:50:22]   expected: generic_drop_glue0[Internal] 
[00:50:22]   actual:   generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<i8, char>>
[00:50:22]   expected: generic_drop_glue0[Internal] 
[00:50:22]   actual:   generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/generic-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<instantiation_through_vtable::Struct[0]<u32>>
[00:50:22]   expected: instantiation_through_vtable0[Internal] 
[00:50:22]   actual:   instantiation_through_vtable.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<instantiation_through_vtable::Struct[0]<u64>>
[00:50:22]   expected: instantiation_through_vtable0[Internal] 
[00:50:22]   actual:   instantiation_through_vtable.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::EnumWithDrop[0]>
[00:50:22]   expected: non_generic_drop_glue0[Internal] 
[00:50:22]   actual:   non_generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::StructWithDrop[0]>
[00:50:22]   expected: non_generic_drop_glue0[Internal] 
[00:50:22]   actual:   non_generic_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<i16>>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<u32>>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Intermediate[0]>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<i16>>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<u32>>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Leaf[0]>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<i16>>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<u32>>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Root[0]>
[00:50:22]   expected: transitive_drop_glue0[Internal] 
[00:50:22]   actual:   transitive_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/transitive-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<(i16, (tuple_drop_glue::Dropped[0], bool))>
[00:50:22]   expected: tuple_drop_glue0[Internal] 
[00:50:22]   actual:   tuple_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<(tuple_drop_glue::Dropped[0], bool)>
[00:50:22]   expected: tuple_drop_glue0[Internal] 
[00:50:22]   actual:   tuple_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<(u32, tuple_drop_glue::Dropped[0])>
[00:50:22]   expected: tuple_drop_glue0[Internal] 
[00:50:22]   actual:   tuple_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<tuple_drop_glue::Dropped[0]>
[00:50:22]   expected: tuple_drop_glue0[Internal] 
[00:50:22]   actual:   tuple_drop_glue.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/tuple-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<bool>
[00:50:22]   expected: unsizing0[Internal] 
[00:50:22]   actual:   unsizing.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<char>
[00:50:22]   expected: unsizing0[Internal] 
[00:50:22]   actual:   unsizing.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<f64>
[00:50:22]   expected: unsizing0[Internal] 
[00:50:22]   actual:   unsizing.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<u32>
[00:50:22]   expected: unsizing0[Internal] 
[00:50:22]   actual:   unsizing.7rcbfp3gighlevaayutqt3mda-cgu.0[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/item-collection/unsizing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
[00:50:22]   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
[00:50:22]   actual:   extern_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] extern_drop_glue.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
[00:50:22]   expected: extern_drop_glue[Internal] 
[00:50:22]   actual:   extern_drop_glue.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
[00:50:22]   expected: extern_drop_glue-mod1[Internal] 
[00:50:22]   actual:   extern_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn extern_drop_glue::mod1[0]::user[0]
[00:50:22]   expected: extern_drop_glue-mod1[External] 
[00:50:22]   actual:   extern_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[External] 
[00:50:22] 
[00:50:22] fn extern_drop_glue::user[0]
[00:50:22]   expected: extern_drop_glue[External] 
[00:50:22]   actual:   extern_drop_glue.3a1fbbbhnueig6z2i34ldilf1[External] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
[00:50:22]   expected: local_drop_glue-mod1[Internal] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]>
[00:50:22]   expected: local_drop_glue[Internal] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]>
[00:50:22]   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] local_drop_glue.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
[00:50:22]   expected: local_drop_glue-mod1[Internal] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn local_drop_glue::mod1[0]::user[0]
[00:50:22]   expected: local_drop_glue-mod1[External] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1-mod1[External] 
[00:50:22] 
[00:50:22] fn local_drop_glue::user[0]
[00:50:22]   expected: local_drop_glue[External] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1[External] 
[00:50:22] 
[00:50:22] fn local_drop_glue::{{impl}}[0]::drop[0]
[00:50:22]   expected: local_drop_glue[External] 
[00:50:22]   actual:   local_drop_glue.3a1fbbbhnueig6z2i34ldilf1[External] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn cgu_generic_function::bar[0]<&str>
[00:50:22]   expected: cgu_generic_function.volatile[External] 
[00:50:22]   actual:   cgu_generic_function.3a1fbbbhnueig6z2i34ldilf1.volatile[External] 
[00:50:22] 
[00:50:22] fn cgu_generic_function::foo[0]<&str>
[00:50:22]   expected: cgu_generic_function.volatile[External] 
[00:50:22]   actual:   cgu_generic_function.3a1fbbbhnueig6z2i34ldilf1.volatile[External] 
[00:50:22] 
[00:50:22] fn extern_generic::mod1[0]::mod1[0]::user[0]
[00:50:22]   expected: extern_generic-mod1-mod1[Internal] 
[00:50:22]   actual:   extern_generic.3a1fbbbhnueig6z2i34ldilf1-mod1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn extern_generic::mod1[0]::user[0]
[00:50:22]   expected: extern_generic-mod1[Internal] 
[00:50:22]   actual:   extern_generic.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn extern_generic::mod2[0]::user[0]
[00:50:22]   expected: extern_generic-mod2[Internal] 
[00:50:22]   actual:   extern_generic.3a1fbbbhnueig6z2i34ldilf1-mod2[Internal] 
[00:50:22] 
[00:50:22] fn extern_generic::mod3[0]::non_user[0]
[00:50:22]   expected: extern_generic-mod3[Internal] 
[00:50:22]   actual:   extern_generic.3a1fbbbhnueig6z2i34ldilf1-mod3[Internal] 
[00:50:22] 
[00:50:22] fn extern_generic::user[0]
[00:50:22]   expected: extern_generic[Internal] 
[00:50:22]   actual:   extern_generic.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn cgu_explicit_inlining::always_inlined[0]
[00:50:22]   expected: inlining_from_extern_crate-mod2[Internal] inlining_from_extern_crate[Internal] 
[00:50:22]   actual:   inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1-mod2[Internal] inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn cgu_explicit_inlining::inlined[0]
[00:50:22]   expected: inlining_from_extern_crate-mod1[Internal] inlining_from_extern_crate[Internal] 
[00:50:22]   actual:   inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn inlining_from_extern_crate::mod1[0]::user[0]
[00:50:22]   expected: inlining_from_extern_crate-mod1[External] 
[00:50:22]   actual:   inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1-mod1[External] 
[00:50:22] 
[00:50:22] fn inlining_from_extern_crate::mod2[0]::user[0]
[00:50:22]   expected: inlining_from_extern_crate-mod2[External] 
[00:50:22]   actual:   inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1-mod2[External] 
[00:50:22] fn inlining_from_extern_crate::user[0]
[00:50:22] fn inlining_from_extern_crate::user[0]
[00:50:22]   expected: inlining_from_extern_crate[External] 
[00:50:22]   actual:   inlining_from_extern_crate.3a1fbbbhnueig6z2i34ldilf1[External] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn local_inlining_but_not_all::inline[0]::inlined_function[0]
[00:50:22]   expected: local_inlining_but_not_all-inline[External] 
[00:50:22]   actual:   local_inlining_but_not_all.3a1fbbbhnueig6z2i34ldilf1-inline[External] 
[00:50:22] 
[00:50:22] fn local_inlining_but_not_all::non_user[0]::baz[0]
[00:50:22]   expected: local_inlining_but_not_all-non_user[External] 
[00:50:22]   actual:   local_inlining_but_not_all.3a1fbbbhnueig6z2i34ldilf1-non_user[External] 
[00:50:22] 
[00:50:22] fn local_inlining_but_not_all::user1[0]::foo[0]
[00:50:22]   expected: local_inlining_but_not_all-user1[External] 
[00:50:22]   actual:   local_inlining_but_not_all.3a1fbbbhnueig6z2i34ldilf1-user1[External] 
[00:50:22] 
[00:50:22] fn local_inlining_but_not_all::user2[0]::bar[0]
[00:50:22]   expected: local_inlining_but_not_all-user2[External] 
[00:50:22]   actual:   local_inlining_but_not_all.3a1fbbbhnueig6z2i34ldilf1-user2[External] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn local_generic::generic[0]<&str>
[00:50:22]   expected: local_generic.volatile[External] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1.volatile[External] 
[00:50:22] 
[00:50:22] fn local_generic::generic[0]<char>
[00:50:22]   expected: local_generic.volatile[External] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1.volatile[External] 
[00:50:22] 
[00:50:22] fn local_generic::generic[0]<u32>
[00:50:22]   expected: local_generic.volatile[External] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1.volatile[External] 
[00:50:22] 
[00:50:22] fn local_generic::generic[0]<u64>
[00:50:22]   expected: local_generic.volatile[External] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1.volatile[External] 
[00:50:22] 
[00:50:22] fn local_generic::mod1[0]::mod1[0]::user[0]
[00:50:22]   expected: local_generic-mod1-mod1[Internal] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1-mod1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn local_generic::mod1[0]::user[0]
[00:50:22]   expected: local_generic-mod1[Internal] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn local_generic::mod2[0]::user[0]
[00:50:22]   expected: local_generic-mod2[Internal] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1-mod2[Internal] 
[00:50:22] 
[00:50:22] fn local_generic::user[0]
[00:50:22]   expected: local_generic[Internal] 
[00:50:22]   actual:   local_generic.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn local_inlining::inline[0]::inlined_function[0]
[00:50:22]   expected: local_inlining-user1[Internal] local_inlining-user2[Internal] 
[00:50:22]   actual:   local_inlining.3a1fbbbhnueig6z2i34ldilf1-user1[Internal] local_inlining.3a1fbbbhnueig6z2i34ldilf1-user2[Internal] 
[00:50:22] 
[00:50:22] fn local_inlining::non_user[0]::baz[0]
[00:50:22]   expected: local_inlining-non_user[External] 
[00:50:22]   actual:   local_inlining.3a1fbbbhnueig6z2i34ldilf1-non_user[External] 
[00:50:22] 
[00:50:22] fn local_inlining::user1[0]::foo[0]
[00:50:22]   expected: local_inlining-user1[External] 
[00:50:22]   actual:   local_inlining.3a1fbbbhnueig6z2i34ldilf1-user1[External] 
[00:50:22] 
[00:50:22] fn local_inlining::user2[0]::bar[0]
[00:50:22]   expected: local_inlining-user2[External] 
[00:50:22]   actual:   local_inlining.3a1fbbbhnueig6z2i34ldilf1-user2[External] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] 
[00:50:22] fn local_transitive_inlining::direct_user[0]::foo[0]
[00:50:22]   expected: local_transitive_inlining-indirect_user[Internal] 
[00:50:22]   actual:   local_transitive_inlining.3a1fbbbhnueig6z2i34ldilf1-indirect_user[Internal] 
[00:50:22] 
[00:50:22] fn local_transitive_inlining::indirect_user[0]::bar[0]
[00:50:22]   expected: local_transitive_inlining-indirect_user[External] 
[00:50:22]   actual:   local_transitive_inlining.3a1fbbbhnueig6z2i34ldilf1-indirect_user[External] 
[00:50:22] 
[00:50:22] fn local_transitive_inlining::inline[0]::inlined_function[0]
[00:50:22]   expected: local_transitive_inlining-indirect_user[Internal] 
[00:50:22]   actual:   local_transitive_inlining.3a1fbbbhnueig6z2i34ldilf1-indirect_user[Internal] 
[00:50:22] 
[00:50:22] fn local_transitive_inlining::non_user[0]::baz[0]
[00:50:22]   expected: local_transitive_inlining-non_user[External] 
[00:50:22]   actual:   local_transitive_inlining.3a1fbbbhnueig6z2i34ldilf1-non_user[External] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] fn regular_modules::bar[0]
[00:50:22] fn regular_modules::bar[0]
[00:50:22]   expected: regular_modules[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::foo[0]
[00:50:22]   expected: regular_modules[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod1[0]::bar[0]
[00:50:22]   expected: regular_modules-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod1[0]::foo[0]
[00:50:22]   expected: regular_modules-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod1[0]::mod1[0]::bar[0]
[00:50:22]   expected: regular_modules-mod1-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod1[0]::mod1[0]::foo[0]
[00:50:22]   expected: regular_modules-mod1-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1-mod1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod1[0]::mod2[0]::bar[0]
[00:50:22]   expected: regular_modules-mod1-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1-mod2[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod1[0]::mod2[0]::foo[0]
[00:50:22]   expected: regular_modules-mod1-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1-mod2[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod2[0]::bar[0]
[00:50:22]   expected: regular_modules-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod2[0]::foo[0]
[00:50:22]   expected: regular_modules-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod2[0]::mod1[0]::bar[0]
[00:50:22]   expected: regular_modules-mod2-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2-mod1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod2[0]::mod1[0]::foo[0]
[00:50:22]   expected: regular_modules-mod2-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2-mod1[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod2[0]::mod2[0]::bar[0]
[00:50:22]   expected: regular_modules-mod2-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2-mod2[Internal] 
[00:50:22] 
[00:50:22] fn regular_modules::mod2[0]::mod2[0]::foo[0]
[00:50:22]   expected: regular_modules-mod2-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2-mod2[Internal] 
[00:50:22] static regular_modules::BAZ[0]
[00:50:22] static regular_modules::BAZ[0]
[00:50:22]   expected: regular_modules[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1[Internal] 
[00:50:22] 
[00:50:22] static regular_modules::mod1[0]::BAZ[0]
[00:50:22]   expected: regular_modules-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1[Internal] 
[00:50:22] 
[00:50:22] static regular_modules::mod1[0]::mod1[0]::BAZ[0]
[00:50:22]   expected: regular_modules-mod1-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1-mod1[Internal] 
[00:50:22] 
[00:50:22] static regular_modules::mod1[0]::mod2[0]::BAZ[0]
[00:50:22]   expected: regular_modules-mod1-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod1-mod2[Internal] 
[00:50:22] 
[00:50:22] static regular_modules::mod2[0]::BAZ[0]
[00:50:22]   expected: regular_modules-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2[Internal] 
[00:50:22] 
[00:50:22] static regular_modules::mod2[0]::mod1[0]::BAZ[0]
[00:50:22]   expected: regular_modules-mod2-mod1[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2-mod1[Internal] 
[00:50:22] 
[00:50:22] static regular_modules::mod2[0]::mod2[0]::BAZ[0]
[00:50:22]   expected: regular_modules-mod2-mod2[Internal] 
[00:50:22]   actual:   regular_modules.3a1fbbbhnueig6z2i34ldilf1-mod2-mod2[Internal] 
[00:50:22] thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2292:13
[00:50:22] 
[00:50:22] ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
[00:50:22] 
[00:50:22] 
[00:50:22] The following items were assigned to wrong codegen units:
[00:50:22] fn statics::function[0]
[00:50:22] fn statics::function[0]
[00:50:22]   expected: statics[External] 
[00:50:22]   actual:   statics.3a1fbbbhnueig6z2i34ldilf1[External] 
[00:50:22] 
[00:50:22] fn statics::mod1[0]::function[0]
[00:50:22]   expected: statics-mod1[External] 
[00:50:22]   actual:   statics.3a1fbbbhnueig6z2i34ldilf1-mod1[External] 
[00:50:22] static statics::BAR[0]
[00:50:22] static statics::BAR[0]
[00:50:22]   expected: statics[Internal] 
[00:50:22]   actual:   statics.3a1fbbbhnueig6z2i34ldilf1[Internal] 
---
[00:50:22] test result: FAILED. 17 passed; 19 failed; 3 ignored; 0 measured; 0 filtered out
[00:50:22] 
[00:50:22] 
[00:50:22] 
[00:50:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" 

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c2f6d5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:038102dc:start=1531341981212667140,finish=1531341981219137866,duration=6470726
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04afa40e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b933198
$ dmesg | grep -i kill
