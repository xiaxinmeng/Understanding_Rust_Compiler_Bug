plain
 finished in 0.816 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 162 tests
....F.................................F.................................................  88/162
.........F..........................................F.....................
failures:

---- [incremental] tests/incremental/change_private_fn/struct_point.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary" "-Z" "query-dep-graph"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/index.crates.io-6f17d22bba15001f/scoped-tls-1.0.0/src/lib.rs:168:9
   0:     0x7f9837212334 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7bdcf5daed3df079
   1:     0x7f9837278e98 - core::fmt::write::h8677bbea7dac8727
   2:     0x7f9837206b41 - std::io::Write::write_fmt::h5929f888bbd07d15
   3:     0x7f9837212141 - std::sys_common::backtrace::print::hff419bf84e299d47
   3:     0x7f9837212141 - std::sys_common::backtrace::print::hff419bf84e299d47
   4:     0x7f98372152ca - std::panicking::default_hook::{{closure}}::h3d0edbc21d00763c
   5:     0x7f9837214fac - std::panicking::default_hook::h317c9f3eba0442a8
   6:     0x7f9837ce10cb - rustc_driver_impl[a56b617e4323871b]::install_ice_hook::{closure#0}
   7:     0x7f98372159d7 - std::panicking::rust_panic_with_hook::heebc98c780568d7e
   7:     0x7f98372159d7 - std::panicking::rust_panic_with_hook::heebc98c780568d7e
   8:     0x7f983aee4f5a - std[e934fefb7b2382f5]::panicking::begin_panic::<&str>::{closure#0}
   9:     0x7f983aee47d6 - std[e934fefb7b2382f5]::sys_common::backtrace::__rust_end_short_backtrace::<std[e934fefb7b2382f5]::panicking::begin_panic<&str>::{closure#0}, !>
  10:     0x7f9837cc875a - std[e934fefb7b2382f5]::panicking::begin_panic::<&str>
  11:     0x7f983aee28c3 - <scoped_tls[e5a9e0b2452e3da0]::ScopedKey<rustc_span[30ffe155388182ca]::SessionGlobals>>::with::<<rustc_span[30ffe155388182ca]::symbol::Symbol>::intern::{closure#0}, rustc_span[30ffe155388182ca]::symbol::Symbol>
  12:     0x7f9839cc9a1b - rustc_incremental[1a2f6d97c3b3f110]::persist::file_format::rustc_version
  13:     0x7f9839cc9596 - rustc_incremental[1a2f6d97c3b3f110]::persist::file_format::read_file
  14:     0x7f9839cda6ac - rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_data_no_sess
  15:     0x7f9839c9e0f0 - std[e934fefb7b2382f5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>
  16:     0x7f9839cabf03 - std[e934fefb7b2382f5]::panicking::try::<rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>, core[c169c69c2b50982d]::panic::unwind_safe::AssertUnwindSafe<<std[e934fefb7b2382f5]::thread::Builder>::spawn_unchecked_<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>::{closure#1}::{closure#0}>>
  17:     0x7f9839c9ef44 - <<std[e934fefb7b2382f5]::thread::Builder>::spawn_unchecked_<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>::{closure#1} as core[c169c69c2b50982d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  18:     0x7f98372222ee - std::sys::unix::thread::Thread::new::thread_start::h04d19f5cfbc9fa04
  19:     0x7f9836fbcb43 - <unknown>
  20:     0x7f983704ea00 - <unknown>
  21:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (3824d2956 2023-05-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any { .. }


error: `typeck(fn_calls_methods_in_same_impl::check)` should have been loaded from disk but it was not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^


error: aborting due to previous error; 1 warning emitted
------------------------------------------


---- [incremental] tests/incremental/dirty_clean.rs stdout ----

error in revision `cfail2`: incremental test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/dirty_clean.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary" "-Z" "query-dep-graph"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/index.crates.io-6f17d22bba15001f/scoped-tls-1.0.0/src/lib.rs:168:9
   0:     0x7efefd678334 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7bdcf5daed3df079
   1:     0x7efefd6dee98 - core::fmt::write::h8677bbea7dac8727
   1:     0x7efefd6dee98 - core::fmt::write::h8677bbea7dac8727
   2:     0x7efefd66cb41 - std::io::Write::write_fmt::h5929f888bbd07d15
   3:     0x7efefd678141 - std::sys_common::backtrace::print::hff419bf84e299d47
   4:     0x7efefd67b2ca - std::panicking::default_hook::{{closure}}::h3d0edbc21d00763c
   5:     0x7efefd67afac - std::panicking::default_hook::h317c9f3eba0442a8
   6:     0x7efefe1470cb - rustc_driver_impl[a56b617e4323871b]::install_ice_hook::{closure#0}
   7:     0x7efefd67b9d7 - std::panicking::rust_panic_with_hook::heebc98c780568d7e
   8:     0x7eff0134af5a - std[e934fefb7b2382f5]::panicking::begin_panic::<&str>::{closure#0}
   9:     0x7eff0134a7d6 - std[e934fefb7b2382f5]::sys_common::backtrace::__rust_end_short_backtrace::<std[e934fefb7b2382f5]::panicking::begin_panic<&str>::{closure#0}, !>
  10:     0x7efefe12e75a - std[e934fefb7b2382f5]::panicking::begin_panic::<&str>
  11:     0x7eff013488c3 - <scoped_tls[e5a9e0b2452e3da0]::ScopedKey<rustc_span[30ffe155388182ca]::SessionGlobals>>::with::<<rustc_span[30ffe155388182ca]::symbol::Symbol>::intern::{closure#0}, rustc_span[30ffe155388182ca]::symbol::Symbol>
  12:     0x7eff0012fa1b - rustc_incremental[1a2f6d97c3b3f110]::persist::file_format::rustc_version
  13:     0x7eff0012f596 - rustc_incremental[1a2f6d97c3b3f110]::persist::file_format::read_file
  14:     0x7eff001406ac - rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_data_no_sess
  15:     0x7eff001040f0 - std[e934fefb7b2382f5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>
  16:     0x7eff00111f03 - std[e934fefb7b2382f5]::panicking::try::<rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>, core[c169c69c2b50982d]::panic::unwind_safe::AssertUnwindSafe<<std[e934fefb7b2382f5]::thread::Builder>::spawn_unchecked_<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>::{closure#1}::{closure#0}>>
  17:     0x7eff00104f44 - <<std[e934fefb7b2382f5]::thread::Builder>::spawn_unchecked_<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>::{closure#1} as core[c169c69c2b50982d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  18:     0x7efefd6882ee - std::sys::unix::thread::Thread::new::thread_start::h04d19f5cfbc9fa04
  19:     0x7efefd422b43 - <unknown>
  20:     0x7efefd4b4a00 - <unknown>
  21:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (3824d2956 2023-05-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any { .. }


warning: 1 warning emitted
------------------------------------------


---- [incremental] tests/incremental/issue-85360-eval-obligation-ice.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/issue-85360-eval-obligation-ice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-85360-eval-obligation-ice/issue-85360-eval-obligation-ice.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-85360-eval-obligation-ice" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-85360-eval-obligation-ice/auxiliary" "--crate-type=lib" "--edition=2021" "-Zassert-incr-state=loaded"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/index.crates.io-6f17d22bba15001f/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7ffb273a9334 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7bdcf5daed3df079
   1:     0x7ffb2740fe98 - core::fmt::write::h8677bbea7dac8727
   2:     0x7ffb2739db41 - std::io::Write::write_fmt::h5929f888bbd07d15
   3:     0x7ffb273a9141 - std::sys_common::backtrace::print::hff419bf84e299d47
   4:     0x7ffb273ac2ca - std::panicking::default_hook::{{closure}}::h3d0edbc21d00763c
   5:     0x7ffb273abfac - std::panicking::default_hook::h317c9f3eba0442a8
   6:     0x7ffb27e780cb - rustc_driver_impl[a56b617e4323871b]::install_ice_hook::{closure#0}
   7:     0x7ffb273ac9d7 - std::panicking::rust_panic_with_hook::heebc98c780568d7e
   8:     0x7ffb2b07bf5a - std[e934fefb7b2382f5]::panicking::begin_panic::<&str>::{closure#0}
   9:     0x7ffb2b07b7d6 - std[e934fefb7b2382f5]::sys_common::backtrace::__rust_end_short_backtrace::<std[e934fefb7b2382f5]::panicking::begin_panic<&str>::{closure#0}, !>
  10:     0x7ffb27e5f75a - std[e934fefb7b2382f5]::panicking::begin_panic::<&str>
  11:     0x7ffb2b0798c3 - <scoped_tls[e5a9e0b2452e3da0]::ScopedKey<rustc_span[30ffe155388182ca]::SessionGlobals>>::with::<<rustc_span[30ffe155388182ca]::symbol::Symbol>::intern::{closure#0}, rustc_span[30ffe155388182ca]::symbol::Symbol>
  12:     0x7ffb29e60a1b - rustc_incremental[1a2f6d97c3b3f110]::persist::file_format::rustc_version
  13:     0x7ffb29e60596 - rustc_incremental[1a2f6d97c3b3f110]::persist::file_format::read_file
  14:     0x7ffb29e716ac - rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_data_no_sess
  15:     0x7ffb29e350f0 - std[e934fefb7b2382f5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>
  16:     0x7ffb29e42f03 - std[e934fefb7b2382f5]::panicking::try::<rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>, core[c169c69c2b50982d]::panic::unwind_safe::AssertUnwindSafe<<std[e934fefb7b2382f5]::thread::Builder>::spawn_unchecked_<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>::{closure#1}::{closure#0}>>
  17:     0x7ffb29e35f44 - <<std[e934fefb7b2382f5]::thread::Builder>::spawn_unchecked_<rustc_incremental[1a2f6d97c3b3f110]::persist::load::load_dep_graph::{closure#1}, rustc_incremental[1a2f6d97c3b3f110]::persist::load::LoadResult<(rustc_query_system[4a09e783ecb13e09]::dep_graph::serialized::SerializedDepGraph<rustc_middle[7908bbb04592216e]::dep_graph::dep_node::DepKind>, std[e934fefb7b2382f5]::collections::hash::map::HashMap<rustc_query_system[4a09e783ecb13e09]::dep_graph::dep_node::WorkProductId, rustc_query_system[4a09e783ecb13e09]::dep_graph::graph::WorkProduct, core[c169c69c2b50982d]::hash::BuildHasherDefault<rustc_hash[27a8caad745f51da]::FxHasher>>)>>::{closure#1} as core[c169c69c2b50982d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  18:     0x7ffb273b92ee - std::sys::unix::thread::Thread::new::thread_start::h04d19f5cfbc9fa04
  19:     0x7ffb27153b43 - <unknown>
  20:     0x7ffb271e5a00 - <unknown>
  21:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (3824d2956 2023-05-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib -Z assert-incr-state=loaded
query stack during panic:
end of query stack
error: we asserted that an existing incremental cache directory should be successfully loaded, but it was not


error: aborting due to previous error
------------------------------------------


---- [incremental] tests/incremental/struct_change_field_name.rs stdout ----

error in revision `cfail2`: /checkout/tests/incremental/struct_change_field_name.rs:30: expected error not found: struct `X` has no field named `x`

error in revision `cfail2`: /checkout/tests/incremental/struct_change_field_name.rs:32: expected error not found: no field `x` on type `X`

error in revision `cfail2`: /checkout/tests/incremental/struct_change_field_name.rs:38: expected error not found: no field `x` on type `X`

error in revision `cfail2`: 0 unexpected errors found, 3 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/struct_change_field_name.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_name/struct_change_field_name.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_name" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_name/auxiliary" "-Z" "query-dep-graph" "-Z" "query-dep-graph" "-Z" "assert-incr-state=loaded"
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "struct `X` has no field named `x`",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "no field `x` on type `X`",
    Error {
        line_num: 38,
        kind: Some(
            Error,
            Error,
        ),
        msg: "no field `x` on type `X`",
]

thread '[incremental] tests/incremental/struct_change_field_name.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1455:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
