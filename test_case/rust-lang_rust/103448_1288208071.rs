plain
 finished in 4.397 seconds
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 42 tests
Some tests failed in compiletest suite=codegen-units mode=codegen-units host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ii.................F.iF...................

---- [codegen-units] src/test/codegen-units/item-collection/trait-implementations.rs stdout ----

These items should have been contained but were not:
These items should have been contained but were not:

MONO_ITEM fn <f32 as SomeGenericTrait<&str>>::bar::<&str>
MONO_ITEM fn <f32 as SomeGenericTrait<u32>>::bar::<()>
MONO_ITEM fn <f64 as SomeGenericTrait<u32>>::bar::<&str>
MONO_ITEM fn <f64 as SomeGenericTrait<u32>>::bar::<()>
MONO_ITEM fn <i32 as SomeTrait>::bar::<char>


These items were contained but should not have been:


MONO_ITEM fn <f32 as SomeGenericTrait<&str>>::bar::<&str, '_> @@ trait_implementations.3404cb38-cgu.0[Internal]
MONO_ITEM fn <f32 as SomeGenericTrait<u32>>::bar::<(), '_> @@ trait_implementations.3404cb38-cgu.0[Internal]
MONO_ITEM fn <f64 as SomeGenericTrait<u32>>::bar::<&str, '_> @@ trait_implementations.3404cb38-cgu.0[Internal]
MONO_ITEM fn <f64 as SomeGenericTrait<u32>>::bar::<(), '_> @@ trait_implementations.3404cb38-cgu.0[Internal]
MONO_ITEM fn <i32 as SomeTrait>::bar::<char, '_> @@ trait_implementations.3404cb38-cgu.0[Internal]

thread '[codegen-units] src/test/codegen-units/item-collection/trait-implementations.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2774:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


---- [codegen-units] src/test/codegen-units/item-collection/trait-method-default-impl.rs stdout ----

These items should have been contained but were not:

MONO_ITEM fn <i32 as SomeGenericTrait<u64>>::bar::<&str>
MONO_ITEM fn <i32 as SomeGenericTrait<u64>>::bar::<char>
MONO_ITEM fn <i8 as SomeTrait>::bar::<&str>
MONO_ITEM fn <i8 as SomeTrait>::bar::<char>
MONO_ITEM fn <u32 as SomeGenericTrait<i16>>::bar::<()>
MONO_ITEM fn <u32 as SomeGenericTrait<i8>>::bar::<&[char; 1]>


These items were contained but should not have been:


MONO_ITEM fn <i32 as SomeGenericTrait<u64>>::bar::<&str, '_> @@ trait_method_default_impl.519c9d32-cgu.0[Internal]
MONO_ITEM fn <i32 as SomeGenericTrait<u64>>::bar::<char, '_> @@ trait_method_default_impl.519c9d32-cgu.0[Internal]
MONO_ITEM fn <i8 as SomeTrait>::bar::<&str, '_> @@ trait_method_default_impl.519c9d32-cgu.0[Internal]
MONO_ITEM fn <i8 as SomeTrait>::bar::<char, '_> @@ trait_method_default_impl.519c9d32-cgu.0[Internal]
MONO_ITEM fn <u32 as SomeGenericTrait<i16>>::bar::<(), '_> @@ trait_method_default_impl.519c9d32-cgu.0[Internal]
MONO_ITEM fn <u32 as SomeGenericTrait<i8>>::bar::<&[char; 1], '_> @@ trait_method_default_impl.519c9d32-cgu.0[Internal]

thread '[codegen-units] src/test/codegen-units/item-collection/trait-method-default-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2774:13


