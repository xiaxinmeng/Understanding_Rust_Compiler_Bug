
failures:

---- [pretty] pretty/coerce-expect-unsized.rs stdout ----

error: pretty-printed source (expanded) does not typecheck
status: exit code: 101
command: i686-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --target=i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/run-pass/ -L i686-unknown-linux-gnu/test/run-pass/coerce-expect-unsized.stage2-i686-unknown-linux-gnu.pretty.libaux --cfg rtopt -C rpath -O -L i686-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:74:73: 74:100 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:74                                                                        [::std::fmt::ArgumentV1::new(__arg0,
                                                                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:74:73: 74:100 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:63:37: 63:66 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:63                  ::std::fmt::format(::std::fmt::Arguments::new_v1({
                                              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:63:37: 63:66 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:18:5: 18:25 warning: unused import, #[warn(unused_imports)] on by default
<anon>:18 use std::prelude::v1::*;
              ^~~~~~~~~~~~~~~~~~~~
<anon>:15:10: 15:26 warning: lint unknown_features has been renamed to unused_features, #[warn(renamed_and_removed_lints)] on by default
<anon>:15 #![allow(unknown_features)]
                   ^~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors

------------------------------------------

thread '[pretty] pretty/coerce-expect-unsized.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-32-opt/build/src/compiletest/runtest.rs:1679
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [pretty] pretty/deriving-via-extension-hash-enum.rs stdout ----

error: pretty-printed source (expanded) does not typecheck
status: exit code: 101
command: i686-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --target=i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/run-pass/ -L i686-unknown-linux-gnu/test/run-pass/deriving-via-extension-hash-enum.stage2-i686-unknown-linux-gnu.pretty.libaux --cfg rtopt -C rpath -O -L i686-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:27:46: 27:83 error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)
<anon>:27                                              ::std::intrinsics::discriminant_value(self)
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:27:46: 27:83 help: add #![feature(core_intrinsics)] to the crate attributes to enable
<anon>:34:46: 34:83 error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)
<anon>:34                                              ::std::intrinsics::discriminant_value(self)
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:34:46: 34:83 help: add #![feature(core_intrinsics)] to the crate attributes to enable
<anon>:51:46: 51:83 error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)
<anon>:51                                              ::std::intrinsics::discriminant_value(self)
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:51:46: 51:83 help: add #![feature(core_intrinsics)] to the crate attributes to enable
<anon>:56:46: 56:83 error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)
<anon>:56                                              ::std::intrinsics::discriminant_value(self)
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:56:46: 56:83 help: add #![feature(core_intrinsics)] to the crate attributes to enable
<anon>:61:46: 61:83 error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)
<anon>:61                                              ::std::intrinsics::discriminant_value(self)
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:61:46: 61:83 help: add #![feature(core_intrinsics)] to the crate attributes to enable
<anon>:66:46: 66:83 error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)
<anon>:66                                              ::std::intrinsics::discriminant_value(self)
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:66:46: 66:83 help: add #![feature(core_intrinsics)] to the crate attributes to enable
<anon>:4:5: 4:25 warning: unused import, #[warn(unused_imports)] on by default
<anon>:4 use std::prelude::v1::*;
             ^~~~~~~~~~~~~~~~~~~~
<anon>:19:1: 19:49 warning: enum is never used: `Foo`, #[warn(dead_code)] on by default
<anon>:19 enum Foo { Bar(isize, char), Baz(char, isize), }
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:20:1: 20:25 warning: unused attribute, #[warn(unused_attributes)] on by default
<anon>:20 #[automatically_derived]
          ^~~~~~~~~~~~~~~~~~~~~~~~
<anon>:43:1: 43:23 warning: enum is never used: `A`, #[warn(dead_code)] on by default
<anon>:43 enum A { B, C, D, E, }
          ^~~~~~~~~~~~~~~~~~~~~~
<anon>:44:1: 44:25 warning: unused attribute, #[warn(unused_attributes)] on by default
<anon>:44 #[automatically_derived]
          ^~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 6 previous errors

------------------------------------------

thread '[pretty] pretty/deriving-via-extension-hash-enum.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-32-opt/build/src/compiletest/runtest.rs:1679

---- [pretty] pretty/foreign-dupe.rs stdout ----

error: pretty-printed source (expanded) does not typecheck
status: exit code: 101
command: i686-unknown-linux-gnu/stage2/bin/rustc - -Zno-trans --target=i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/run-pass/ -L i686-unknown-linux-gnu/test/run-pass/foreign-dupe.stage2-i686-unknown-linux-gnu.pretty.libaux --cfg rtopt -C rpath -O -L i686-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:79:93: 79:120 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:79                                                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:79:93: 79:120 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:81:93: 81:120 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:81                                                                                             ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:81:93: 81:120 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:64:57: 64:86 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:64                             ::std::rt::begin_unwind_fmt(::std::fmt::Arguments::new_v1({
                                                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:64:57: 64:86 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:64:29: 64:56 error: use of unstable library feature 'libstd_sys_internals': used by the panic! macro (see issue #0)
<anon>:64                             ::std::rt::begin_unwind_fmt(::std::fmt::Arguments::new_v1({
                                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:64:29: 64:56 help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
<anon>:117:93: 117:120 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:117                                                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:117:93: 117:120 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:119:93: 119:120 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:119                                                                                             ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:119:93: 119:120 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:102:57: 102:86 error: use of unstable library feature 'fmt_internals': internal to format_args! (see issue #0)
<anon>:102                             ::std::rt::begin_unwind_fmt(::std::fmt::Arguments::new_v1({
                                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:102:57: 102:86 help: add #![feature(fmt_internals)] to the crate attributes to enable
<anon>:102:29: 102:56 error: use of unstable library feature 'libstd_sys_internals': used by the panic! macro (see issue #0)
<anon>:102                             ::std::rt::begin_unwind_fmt(::std::fmt::Arguments::new_v1({
                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:102:29: 102:56 help: add #![feature(libstd_sys_internals)] to the crate attributes to enable
<anon>:19:5: 19:25 warning: unused import, #[warn(unused_imports)] on by default
<anon>:19 use std::prelude::v1::*;
              ^~~~~~~~~~~~~~~~~~~~
<anon>:25:9: 25:29 warning: unused import, #[warn(unused_imports)] on by default
<anon>:25     use std::prelude::v1::*;
                  ^~~~~~~~~~~~~~~~~~~~
<anon>:36:9: 36:29 warning: unused import, #[warn(unused_imports)] on by default
<anon>:36     use std::prelude::v1::*;
                  ^~~~~~~~~~~~~~~~~~~~
<anon>:46:9: 46:29 warning: unused import, #[warn(unused_imports)] on by default
<anon>:46     use std::prelude::v1::*;
                  ^~~~~~~~~~~~~~~~~~~~
error: aborting due to 8 previous errors

------------------------------------------

thread '[pretty] pretty/foreign-dupe.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-32-opt/build/src/compiletest/runtest.rs:1679


failures:
    [pretty] pretty/coerce-expect-unsized.rs
    [pretty] pretty/deriving-via-extension-hash-enum.rs
    [pretty] pretty/foreign-dupe.rs
