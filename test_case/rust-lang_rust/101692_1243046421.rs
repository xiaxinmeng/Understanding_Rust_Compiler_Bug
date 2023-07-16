plain
---- [codegen-units] src/test/codegen-units/item-collection/generic-impl.rs stdout ----

These items should have been contained but were not:

MONO_ITEM fn LifeTimeOnly::bar
MONO_ITEM fn LifeTimeOnly::baz
MONO_ITEM fn LifeTimeOnly::foo


These items were contained but should not have been:


MONO_ITEM fn LifeTimeOnly::<'_>::bar @@ generic_impl.bf38ac37-cgu.0[Internal]
MONO_ITEM fn LifeTimeOnly::<'_>::baz @@ generic_impl.bf38ac37-cgu.0[Internal]
MONO_ITEM fn LifeTimeOnly::<'_>::foo @@ generic_impl.bf38ac37-cgu.0[Internal]

thread '[codegen-units] src/test/codegen-units/item-collection/generic-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2775:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

