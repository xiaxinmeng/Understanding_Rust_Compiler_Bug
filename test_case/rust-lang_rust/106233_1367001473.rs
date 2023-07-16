plain
---- [codegen-units] src/test/codegen-units/polymorphization/unused_type_parameters.rs stdout ----

These items should have been contained but were not:

MONO_ITEM fn closures::used_substs::<u32>
MONO_ITEM fn closures::used_substs::<u32>::{closure#0}
MONO_ITEM fn closures::used_substs::<u64>
MONO_ITEM fn closures::used_substs::<u64>::{closure#0}
MONO_ITEM fn functions::used_substs::<u32>
MONO_ITEM fn functions::used_substs::<u64>
MONO_ITEM fn methods::Foo::<u32>::closure_used_substs
MONO_ITEM fn methods::Foo::<u32>::closure_used_substs::{closure#0}
MONO_ITEM fn methods::Foo::<u32>::used_substs
MONO_ITEM fn methods::Foo::<u64>::closure_used_substs
MONO_ITEM fn methods::Foo::<u64>::closure_used_substs::{closure#0}
MONO_ITEM fn methods::Foo::<u64>::used_substs


These items were contained but should not have been:


MONO_ITEM fn closures::used_substs::<T> @@ unused_type_parameters.0169ce5a-cgu.1[External]
MONO_ITEM fn closures::used_substs::<T>::{closure#0} @@ unused_type_parameters.0169ce5a-cgu.1[Internal]
MONO_ITEM fn functions::used_substs::<T> @@ unused_type_parameters.0169ce5a-cgu.2[External]
MONO_ITEM fn methods::Foo::<F>::closure_used_substs @@ unused_type_parameters.0169ce5a-cgu.3[External]
MONO_ITEM fn methods::Foo::<F>::closure_used_substs::{closure#0} @@ unused_type_parameters.0169ce5a-cgu.3[Internal]
MONO_ITEM fn methods::Foo::<F>::used_substs @@ unused_type_parameters.0169ce5a-cgu.3[External]

thread '[codegen-units] src/test/codegen-units/polymorphization/unused_type_parameters.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2773:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

