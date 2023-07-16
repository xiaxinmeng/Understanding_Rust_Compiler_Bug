
---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----



error: compilation failed!

status: exit code: 101

command: x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/codegen-units/partitioning/extern-generic.rs -L x86_64-unknown-linux-gnu/test/codegen-units/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic.stage2-x86_64-unknown-linux-gnu.codegen-units.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic.stage2-x86_64-unknown-linux-gnu --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt -Zprint-trans-items=eager -Zincremental=tmp/partitioning-tests/extern-generic

stdout:

------------------------------------------

TRANS_ITEM drop-glue i8 @@ extern_generic[Internal] extern_generic-mod1[Internal] extern_generic-mod1-mod1[Internal] extern_generic-mod2[Internal]

TRANS_ITEM fn cgu_generic_function::bar[0]<&str> @@ extern_generic[Internal] extern_generic-mod1[Internal] extern_generic-mod1-mod1[Internal] extern_generic-mod2[Internal]

TRANS_ITEM fn cgu_generic_function::foo[0]<&str> @@ extern_generic[Internal] extern_generic-mod1[Internal] extern_generic-mod1-mod1[Internal] extern_generic-mod2[Internal]

TRANS_ITEM fn extern_generic::mod1[0]::mod1[0]::user[0] @@ extern_generic-mod1-mod1[External]

TRANS_ITEM fn extern_generic::mod1[0]::user[0] @@ extern_generic-mod1[External]

TRANS_ITEM fn extern_generic::mod2[0]::user[0] @@ extern_generic-mod2[External]

TRANS_ITEM fn extern_generic::mod3[0]::non_user[0] @@ extern_generic-mod3[External]

TRANS_ITEM fn extern_generic::user[0] @@ extern_generic[External]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:636

note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

stderr:

------------------------------------------

error: internal compiler error: src/librustc_metadata/decoder.rs:75: lookup_item: id not found: DefIndex(14) in crate "cgu_generic_function" with number 11 

note: the compiler unexpectedly panicked. this is a bug. 

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports 

------------------------------------------

thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', /build/src/tools/compiletest/src/runtest.rs:2353

note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:

    [codegen-units] codegen-units/partitioning/extern-generic.rs
