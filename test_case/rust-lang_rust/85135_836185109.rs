plain
.................................................................................................... 100/11837
........................................i...........ii.............................................. 200/11837
.................................................................................................... 300/11837
.................................................................................................... 400/11837
.......................F............................................................................ 500/11837
...........F...............................................................F........................ 600/11837
.......................F........i............................F...................................... 700/11837
............................................F....................................................... 900/11837
.................................................................................................... 1000/11837
.................................................................................................... 1100/11837
.................................................................................................... 1100/11837
...................i...............................................F..............F................. 1200/11837
..........F......................................................................................... 1300/11837
........................F............iiii.ii.i.............i........................................ 1500/11837
.................................................................................................... 1600/11837
.....................................................i.............................................. 1700/11837
.....................................................i.............................................. 1700/11837
..............................................F................................F.................... 1800/11837
.................................................................................................... 2000/11837
.................................................................................................... 2100/11837
.................................................................................................... 2100/11837
.........................................F........F................................................. 2200/11837
.................................................................................................... 2400/11837
.................................................................................................... 2400/11837
...............................F.....................................................F.............. 2500/11837
.................................................................................................... 2600/11837
..........F........................FF........................F...................................... 2700/11837
.......i.Fi......................................................................................... 2800/11837
...............F.F.FF.FFFF.........F................................................................ 2900/11837
...................iiii..i.......................................................................... 3000/11837
.................................................................F.F...........FFF.FF...F........... 3100/11837
........................................FF.......................................................... 3200/11837
.......................................................................F............................ 3300/11837
.................................................................................................... 3500/11837
.................................................................................................... 3500/11837
................................................F.........................................F......... 3600/11837
............................................i....................................................... 3800/11837
.................................................................................................... 3900/11837
.................................................................................................... 3900/11837
F.............................................................................F..................... 4000/11837
.................................................................................................... 4200/11837
.......................................F............................................................ 4300/11837
.................................F.................................................................. 4400/11837
..................................................................................ii................ 4500/11837
..................................................................................ii................ 4500/11837
.........F..............................................F..............i............................ 4600/11837
.................................................................................................... 4800/11837
...........................................................F........................................ 4900/11837
.................................................................................................... 5000/11837
.................................................................................................... 5000/11837
................................F...................................F............................... 5100/11837
.................................................................................................... 5200/11837
.................................................................................i..i...F........... 5300/11837
.............................................................F...................................... 5500/11837
.................................................................................................... 5600/11837
.................................................................................................... 5700/11837
.................................................................................................... 5800/11837
.................................................................................................... 5800/11837
...........F................................................................................i....... 5900/11837
.................................................................................................... 6000/11837
....................................................................................F............i.. 6100/11837
............F..........F..............................................i............................. 6200/11837
F......................F................F...............F........................................... 6300/11837
............................................................................i....i.................. 6500/11837
..............F.i..........................i........................................................ 6600/11837
...........................................................................i........................ 6700/11837
.................................................................................................... 6800/11837
.................................................................................................... 6800/11837
.................F...........................................................ii..................... 6900/11837
..........................i............................................F....................F....... 7000/11837
........................F..................................................F........................ 7100/11837
..................F................................................................................. 7300/11837
F...........ii................i..i..ii.............................................................. 7400/11837
......................F............................................................................. 7500/11837
.................................................................................................... 7600/11837
---
.................................................................................................... 8400/11837
......................i............................................................................. 8500/11837
.................................................................................................... 8600/11837
.................................................................................................... 8700/11837
......................F.....F.................F..................................................... 8800/11837
.................F.................................................................................. 8900/11837
....................iiii.iiiii...............F................................................ii.... 9000/11837
...........i..................F................................................................F.... 9100/11837
..............................................................F.................................F... 9200/11837
.................................................................................................... 9400/11837
.................................................................................................... 9400/11837
..............F...................F.......................F......................................... 9500/11837
...................................................F.F.........FF................................... 9600/11837
...................................................................................................i 9800/11837
iiiiii..iiiiii.i.................................................................................... 9900/11837
.................................................................................................... 10000/11837
.................................................................................................... 10100/11837
.................................................................................................... 10100/11837
....F............................................................................................... 10200/11837
.................................................................................................... 10300/11837
.................................................................................................... 10400/11837
.F........F................................F............F..........F....................F........... 10500/11837
........F...........F......................................................i........................ 10600/11837
...............i.................................................................................... 10700/11837
..........................F......................F......................FF........F................. 10800/11837
.................................................................................................... 10900/11837
..F.................................................................F.F............................. 11000/11837
.......................................ii........................................................... 11200/11837
.................................................................................................... 11300/11837
.................................................................................................... 11400/11837
.................................................................................................... 11400/11837
....................................................F.......................FF.....................F 11500/11837
F................................................................................................... 11600/11837
.............................ii.............................................F....................... 11800/11837
....................F................
failures:


---- [ui] ui/associated-types/defaults-wf.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-wf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-wf" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-wf/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'SourceAnnotation range `(15, 16)` is bigger than source length `0`', /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:273:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_trait_item_well_formed] checking that `Tr3::Ty` is well-formed
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7fddebdd8ca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7fddebe495ac - core::fmt::write::hf00778d011964c9e
   2:     0x7fddebdc9036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7fddebddd037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7fddebddca44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7fddec6b20ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7fddebddd972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7fddebddd4a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7fddebdd913c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7fddebddd409 - rust_begin_unwind
  10:     0x7fddebddd3bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7fddeec786d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7fddeec75911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7fddec6c0ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7fddec6c499a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7fddec6b6d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7fddec6b4381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7fddec6c94fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7fddec6b44f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7fddec6eeb02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7fddec6efc3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7fddebdea37a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7fdde6aa06db - start_thread
  23:     0x7fddeba6c71f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/async-await/generator-desc.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generator-desc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:11:15
   |
LL |     fun(async {}, async {});
   |               ^^ the expected `async` block
   |                         ^^ expected `async` block, found a different `async` block
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:6:16
   |
   |
LL | async fn one() {}
   |                ^ checked the `Output` of this `async fn`, expected opaque type
  ::: /checkout/src/test/ui/async-await/generator-desc.rs:7:16
   |
   |
LL | async fn two() {}
   |                ^ checked the `Output` of this `async fn`, found opaque type
  ::: /checkout/src/test/ui/async-await/generator-desc.rs:13:16
   |
   |
LL |     fun(one(), two());
   |                ^^^^^ expected opaque type, found a different opaque type
   |
thread 'rustc' panicked at 'SourceAnnotation range `(42, 73)` is bigger than source length `0`', /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:273:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies

------------------------------------------



---- [ui] ui/async-await/issue-72442.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-72442.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Cincremental=tmp/issue-72442" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'SourceAnnotation range `(19, 30)` is bigger than source length `0`', /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:273:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7f62b8a54ca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f62b8ac55ac - core::fmt::write::hf00778d011964c9e
   2:     0x7f62b8a45036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f62b8a59037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f62b8a58a44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f62b932e0ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f62b8a59972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f62b8a594a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f62b8a5513c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f62b8a59409 - rust_begin_unwind
  10:     0x7f62b8a593bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7f62bb8f46d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7f62bb8f1911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7f62b933cca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7f62b934099a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7f62b9332d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7f62b9330381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7f62b93454fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7f62b93304f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7f62b936ab02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7f62b936bc3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7f62b8a6637a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7f62b371c6db - start_thread
  23:     0x7f62b86e871f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/async-await/issues/issue-67893.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-67893.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-67893" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--error-format" "human-annotate-rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-67893/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'SourceAnnotation range `(19, 20)` is bigger than source length `0`', /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:273:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7f3721258ca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f37212c95ac - core::fmt::write::hf00778d011964c9e
   2:     0x7f3721249036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f372125d037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f372125ca44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f3721b320ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f372125d972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f372125d4a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f372125913c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f372125d409 - rust_begin_unwind
  10:     0x7f372125d3bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7f37240f86d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7f37240f5911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7f3721b40ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7f3721b4499a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7f3721b36d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7f3721b34381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7f3721b494fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7f3721b344f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7f3721b6eb02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7f3721b6fc3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7f372126a37a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7f371bf206db - start_thread
  23:     0x7f3720eec71f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/async-await/pin-needed-to-poll.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/pin-needed-to-poll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/pin-needed-to-poll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/pin-needed-to-poll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/async-await/pin-needed-to-poll.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `<impl at /checkout/src/test/ui/async-await/pin-needed-to-poll.rs:38:1: 45:2>::poll`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
             10: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
             12: core::fmt::Write::write_fmt
             12: core::fmt::Write::write_fmt
             13: rustc_infer::infer::InferCtxt::ty_to_string
             14: rustc_typeck::check::method::suggest::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::report_method_error
             15: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             16: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             17: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
             18: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             19: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             20: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
             21: rustc_typeck::check::check::check_fn
             22: rustc_infer::infer::InferCtxtBuilder::enter
             23: rustc_typeck::check::inherited::InheritedBuilder::enter
             24: rustc_typeck::check::typeck
             25: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             26: rustc_data_structures::stack::ensure_sufficient_stack
             27: rustc_query_system::query::plumbing::force_query_with_job
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
             28: rustc_query_system::query::plumbing::get_query_impl
             29: rustc_query_system::query::plumbing::get_query
             30: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             31: rustc_typeck::check::typeck_item_bodies
             32: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             33: rustc_data_structures::stack::ensure_sufficient_stack
             34: rustc_query_system::query::plumbing::force_query_with_job
             35: rustc_query_system::query::plumbing::get_query_impl
             36: rustc_query_system::query::plumbing::get_query
             37: rustc_session::utils::<impl rustc_session::session::Session>::time
             38: rustc_typeck::check_crate
             39: rustc_interface::passes::analysis
             40: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             41: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             42: rustc_data_structures::stack::ensure_sufficient_stack
             43: rustc_query_system::query::plumbing::force_query_with_job
             44: rustc_query_system::query::plumbing::get_query_impl
             45: rustc_query_system::query::plumbing::get_query
             46: rustc_interface::passes::QueryContext::enter
             47: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             48: rustc_span::with_source_map
             49: scoped_tls::ScopedKey<T>::set
             50: rustc_span::with_session_globals
             52: core::ops::function::FnOnce::call_once{{vtable.shim}}
             53: std::sys::unix::thread::Thread::new::thread_start
             54: start_thread
             55: __clone
             55: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7f59bea6dca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f59beade5ac - core::fmt::write::hf00778d011964c9e
   2:     0x7f59bea5e036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f59bea72037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f59bea71a44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f59bf3470ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f59bea72972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f59bea724a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f59bea6e13c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f59bea72409 - rust_begin_unwind
  10:     0x7f59bea723bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7f59c190d6d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7f59c190a911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7f59bf355ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7f59bf35999a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7f59bf34bd0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7f59bf349381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7f59bf35e4fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7f59bf3494f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7f59bf383b02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7f59bf384c3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7f59bea7f37a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7f59b97356db - start_thread
  23:     0x7f59be70171f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/binop/issue-77910-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
   |
   |
LL |     assert_eq!(foo, y);
   |     |
   |     |
   |     for<'r> fn(&'r i32) -> &'r i32 {foo}
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/binop/issue-77910-1.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
error: aborting due to previous error

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/borrowck/move-error-snippets.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-snippets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/auxiliary"
------------------------------------------

---


---- [ui] ui/empty/empty-struct-braces-pat-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-braces-pat-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected tuple struct or tuple variant, found struct variant `E::Empty3`
   |
LL |     Empty3 {}
LL |     Empty3 {}
   |     --------- `E::Empty3` defined here
...
LL |         E::Empty3() => ()
   |         ^^^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`

thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/empty/empty-struct-braces-pat-3.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error


For more information about this error, try `rustc --explain E0532`.

------------------------------------------


---- [ui] ui/empty/empty-struct-unit-pat.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-unit-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/empty/empty-struct-unit-pat.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/error-codes/E0004-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0004-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0004-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0004-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/error-codes/E0004-2.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: rustc_middle::ty::print::Printer::default_print_def_path
             10: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             11: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
             12: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
             14: alloc::fmt::format
             14: alloc::fmt::format
             15: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             16: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             17: rustc_mir_build::thir::pattern::check_match::check_match
             18: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             19: rustc_data_structures::stack::ensure_sufficient_stack
             20: rustc_query_system::query::plumbing::force_query_with_job
             21: rustc_query_system::query::plumbing::get_query_impl
             22: rustc_query_system::query::plumbing::get_query
             23: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             24: rustc_session::utils::<impl rustc_session::session::Session>::time
             26: rustc_session::utils::<impl rustc_session::session::Session>::time
             27: rustc_interface::passes::analysis
             27: rustc_interface::passes::analysis
             28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             30: rustc_data_structures::stack::ensure_sufficient_stack
             31: rustc_query_system::query::plumbing::force_query_with_job
             32: rustc_query_system::query::plumbing::get_query_impl
             33: rustc_query_system::query::plumbing::get_query
             34: rustc_interface::passes::QueryContext::enter
             35: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             36: rustc_span::with_source_map
             37: scoped_tls::ScopedKey<T>::set
             38: rustc_span::with_session_globals
             40: core::ops::function::FnOnce::call_once{{vtable.shim}}
             41: std::sys::unix::thread::Thread::new::thread_start
             42: start_thread
             43: __clone
             43: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7fe0e6d3bca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7fe0e6dac5ac - core::fmt::write::hf00778d011964c9e
   2:     0x7fe0e6d2c036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7fe0e6d40037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7fe0e6d3fa44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7fe0e76150ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7fe0e6d40972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7fe0e6d404a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7fe0e6d3c13c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7fe0e6d40409 - rust_begin_unwind
  10:     0x7fe0e6d403bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7fe0e9bdb6d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7fe0e9bd8911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7fe0e7623ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7fe0e762799a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7fe0e7619d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7fe0e7617381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7fe0e762c4fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7fe0e76174f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7fe0e7651b02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7fe0e7652c3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7fe0e6d4d37a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7fe0e1a036db - start_thread
  23:     0x7fe0e69cf71f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/error-codes/E0005.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0005.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0005" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0005/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/error-codes/E0005.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: rustc_middle::ty::print::Printer::default_print_def_path
             10: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             11: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
             12: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
             14: alloc::fmt::format
             14: alloc::fmt::format
             15: rustc_mir_build::thir::pattern::check_match::MatchVisitor::check_irrefutable
             16: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_local
             17: rustc_hir::intravisit::walk_expr
             18: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             19: rustc_mir_build::thir::pattern::check_match::check_match
             20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             21: rustc_data_structures::stack::ensure_sufficient_stack
             22: rustc_query_system::query::plumbing::force_query_with_job
             23: rustc_query_system::query::plumbing::get_query_impl
             24: rustc_query_system::query::plumbing::get_query
             25: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             26: rustc_session::utils::<impl rustc_session::session::Session>::time
             28: rustc_session::utils::<impl rustc_session::session::Session>::time
             29: rustc_interface::passes::analysis
             29: rustc_interface::passes::analysis
             30: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             31: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             32: rustc_data_structures::stack::ensure_sufficient_stack
             33: rustc_query_system::query::plumbing::force_query_with_job
             34: rustc_query_system::query::plumbing::get_query_impl
             35: rustc_query_system::query::plumbing::get_query
             36: rustc_interface::passes::QueryContext::enter
             37: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             38: rustc_span::with_source_map
             39: scoped_tls::ScopedKey<T>::set
             40: rustc_span::with_session_globals
             42: core::ops::function::FnOnce::call_once{{vtable.shim}}
             43: std::sys::unix::thread::Thread::new::thread_start
             44: start_thread
             45: __clone
             45: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7f112195aca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f11219cb5ac - core::fmt::write::hf00778d011964c9e
   2:     0x7f112194b036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f112195f037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f112195ea44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f11222340ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f112195f972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f112195f4a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f112195b13c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f112195f409 - rust_begin_unwind
  10:     0x7f112195f3bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7f11247fa6d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7f11247f7911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7f1122242ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7f112224699a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7f1122238d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7f1122236381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7f112224b4fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7f11222364f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7f1122270b02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7f1122271c3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7f112196c37a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7f111c6226db - start_thread
  23:     0x7f11215ee71f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/error-codes/E0297.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0297.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0297" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0297/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/error-codes/E0297.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: rustc_middle::ty::print::Printer::default_print_def_path
             10: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
             11: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
             12: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
             14: alloc::fmt::format
             14: alloc::fmt::format
             15: rustc_mir_build::thir::pattern::check_match::MatchVisitor::check_irrefutable
             16: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_local
             17: rustc_hir::intravisit::walk_expr
             18: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             19: rustc_hir::intravisit::walk_expr
             20: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             21: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             22: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             23: rustc_mir_build::thir::pattern::check_match::check_match
             24: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             25: rustc_data_structures::stack::ensure_sufficient_stack
             26: rustc_query_system::query::plumbing::force_query_with_job
             27: rustc_query_system::query::plumbing::get_query_impl
             28: rustc_query_system::query::plumbing::get_query
             29: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             30: rustc_session::utils::<impl rustc_session::session::Session>::time
             32: rustc_session::utils::<impl rustc_session::session::Session>::time
             33: rustc_interface::passes::analysis
             33: rustc_interface::passes::analysis
             34: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             35: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             36: rustc_data_structures::stack::ensure_sufficient_stack
             37: rustc_query_system::query::plumbing::force_query_with_job
             38: rustc_query_system::query::plumbing::get_query_impl
             39: rustc_query_system::query::plumbing::get_query
             40: rustc_interface::passes::QueryContext::enter
             41: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             42: rustc_span::with_source_map
             43: scoped_tls::ScopedKey<T>::set
             44: rustc_span::with_session_globals
             46: core::ops::function::FnOnce::call_once{{vtable.shim}}
             47: std::sys::unix::thread::Thread::new::thread_start
             48: start_thread
             49: __clone
             49: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7fccbd38fca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7fccbd4005ac - core::fmt::write::hf00778d011964c9e
   2:     0x7fccbd380036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7fccbd394037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7fccbd393a44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7fccbdc690ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7fccbd394972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7fccbd3944a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7fccbd39013c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7fccbd394409 - rust_begin_unwind
  10:     0x7fccbd3943bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7fccc022f6d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7fccc022c911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7fccbdc77ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7fccbdc7b99a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7fccbdc6dd0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7fccbdc6b381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7fccbdc804fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7fccbdc6b4f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7fccbdca5b02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7fccbdca6c3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7fccbd3a137a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7fccb80576db - start_thread
  23:     0x7fccbd02371f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-associated_type_bounds.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:15:22
   |
LL |     type A: Iterator<Item: Copy>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:19:22
   |
   |
LL |     type B: Iterator<Item: 'static>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:23:20
   |
   |
LL | struct _St1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:30:18
   |
   |
LL | enum _En1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:37:19
   |
   |
LL | union _Un1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
---
...
LL |         Z;
   |         ^
   |         |
   |         constructor is not visible here due to private fields
   |         help: a tuple struct with a similar name exists: `S`
error[E0423]: expected value, found struct `S`
  --> /checkout/src/test/ui/resolve/privacy-struct-ctor.rs:33:5
   |
LL |     S;
LL |     S;
   |     ^ constructor is not visible here due to private fields
error[E0423]: expected value, found struct `S2`
  --> /checkout/src/test/ui/resolve/privacy-struct-ctor.rs:38:5
   |
LL | /     pub struct S2 {
LL | /     pub struct S2 {
LL | |         s: u8
LL | |     }
   | |_____- `S2` defined here
LL |       S2;
LL |       S2;
   |       ^^ help: use struct literal syntax instead: `S2 { s: val }`

error[E0423]: expected value, found struct `xcrate::S`
   |
LL |     xcrate::S;
LL |     xcrate::S;
   |     ^^^^^^^^^ constructor is not visible here due to private fields
help: consider importing this tuple struct instead
   |
   |
LL | use m::S;


error[E0603]: tuple struct constructor `Z` is private
   |
   |
LL |         pub(in m) struct Z(pub(in m::n) u8);
   |                            --------------- a constructor is private if any of the fields is private
LL |         n::Z;
LL |         n::Z;
   |            ^ private tuple struct constructor
   |
note: the tuple struct constructor `Z` is defined here
   |
   |
LL |         pub(in m) struct Z(pub(in m::n) u8);


error[E0603]: tuple struct constructor `S` is private
   |
LL |     pub struct S(u8);
LL |     pub struct S(u8);
   |                  -- a constructor is private if any of the fields is private
LL |     m::S;
LL |     m::S;
   |        ^ private tuple struct constructor
   |
note: the tuple struct constructor `S` is defined here
   |
LL |     pub struct S(u8);
   |     ^^^^^^^^^^^^^^^^^


error[E0603]: tuple struct constructor `S` is private
   |
LL |     pub struct S(u8);
LL |     pub struct S(u8);
   |                  -- a constructor is private if any of the fields is private
...
LL |     let _: S = m::S(2);
   |                   ^ private tuple struct constructor
   |
note: the tuple struct constructor `S` is defined here
   |
LL |     pub struct S(u8);
   |     ^^^^^^^^^^^^^^^^^


error[E0603]: tuple struct constructor `Z` is private
   |
   |
LL |         pub(in m) struct Z(pub(in m::n) u8);
   |                            --------------- a constructor is private if any of the fields is private
...
LL |     m::n::Z;
   |           ^ private tuple struct constructor
   |
note: the tuple struct constructor `Z` is defined here
   |
   |
LL |         pub(in m) struct Z(pub(in m::n) u8);


thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/resolve/privacy-struct-ctor.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 8 previous errors

---


---- [ui] ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `can_parse_zero_as_f32`
#1 [check_mod_item_types] checking item types in top-level module
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::context::TyCtxt>::def_path_str_with_substs
             10: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::context::TyCtxt>::def_path_str
             11: rustc_trait_selection::traits::on_unimplemented::OnUnimplementedFormatString::format
             12: rustc_trait_selection::traits::on_unimplemented::OnUnimplementedDirective::evaluate
             13: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::on_unimplemented::InferCtxtExt>::on_unimplemented_note
             14: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
             15: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
             16: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
             17: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
             18: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
             19: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
             20: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             21: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             22: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
             23: rustc_typeck::check::check::check_fn
             24: rustc_typeck::check::closure::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_closure
             25: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             26: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             27: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
             28: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
             29: rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
             30: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             31: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             32: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_coercable_to_type
             33: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             34: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             35: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_coercable_to_type
             36: rustc_infer::infer::InferCtxtBuilder::enter
             37: rustc_typeck::check::inherited::InheritedBuilder::enter
             38: rustc_typeck::check::typeck
             39: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             40: rustc_data_structures::stack::ensure_sufficient_stack
             41: rustc_query_system::query::plumbing::force_query_with_job
             42: rustc_query_system::query::plumbing::get_query_impl
             43: rustc_query_system::query::plumbing::get_query
             44: rustc_typeck::check::check::check_item_type
             45: rustc_middle::hir::map::Map::visit_item_likes_in_module
             46: rustc_typeck::check::check::check_mod_item_types
             47: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             48: rustc_data_structures::stack::ensure_sufficient_stack
             49: rustc_query_system::query::plumbing::force_query_with_job
             50: rustc_query_system::query::plumbing::get_query_impl
             51: rustc_query_system::query::plumbing::get_query
             52: rustc_session::utils::<impl rustc_session::session::Session>::time
             53: rustc_typeck::check_crate
             54: rustc_interface::passes::analysis
             55: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             56: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             57: rustc_data_structures::stack::ensure_sufficient_stack
             58: rustc_query_system::query::plumbing::force_query_with_job
             59: rustc_query_system::query::plumbing::get_query_impl
             60: rustc_query_system::query::plumbing::get_query
             61: rustc_interface::passes::QueryContext::enter
             62: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             63: rustc_span::with_source_map
             64: scoped_tls::ScopedKey<T>::set
             65: rustc_span::with_session_globals
             67: core::ops::function::FnOnce::call_once{{vtable.shim}}
             68: std::sys::unix::thread::Thread::new::thread_start
             69: start_thread
             70: __clone
             70: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7f41daa9aca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f41dab0b5ac - core::fmt::write::hf00778d011964c9e
   2:     0x7f41daa8b036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f41daa9f037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f41daa9ea44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f41db3740ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f41daa9f972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f41daa9f4a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f41daa9b13c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f41daa9f409 - rust_begin_unwind
  10:     0x7f41daa9f3bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7f41dd93a6d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7f41dd937911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7f41db382ca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7f41db38699a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7f41db378d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7f41db376381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7f41db38b4fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7f41db3764f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7f41db3b0b02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7f41db3b1c3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7f41daaac37a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7f41d57626db - start_thread
  23:     0x7f41da72e71f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/rfc-2008-non-exhaustive/struct.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2008-non-exhaustive/struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: cannot initialize a tuple struct which contains private fields
  --> /checkout/src/test/ui/rfc-2008-non-exhaustive/struct.rs:20:14
   |
LL |     let ts = TupleStruct(640, 480);
   |              ^^^^^^^^^^^ constructor is not visible here due to private fields

error[E0423]: expected value, found struct `UnitStruct`
   |
   |
LL |     let us = UnitStruct;
   |              ^^^^^^^^^^ constructor is not visible here due to private fields

thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/rfc-2008-non-exhaustive/struct.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors

---


---- [ui] ui/rfc-2008-non-exhaustive/uninhabited/match.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/match/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0004]: non-exhaustive patterns: type `UninhabitedEnum` is non-empty
   |
   |
LL |     match x {} //~ ERROR non-exhaustive patterns
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `UninhabitedEnum`

error[E0004]: non-exhaustive patterns: type `UninhabitedStruct` is non-empty
   |
   |
LL |     match x {} //~ ERROR non-exhaustive patterns
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `UninhabitedStruct`


error[E0004]: non-exhaustive patterns: type `UninhabitedTupleStruct` is non-empty
   |
   |
LL |     match x {} //~ ERROR non-exhaustive patterns
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `UninhabitedTupleStruct`

thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/match.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `cannot_empty_match_on_enum_with_empty_variants_struct_to_anything`
#1 [analysis] running analysis passes on this crate
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0004`.


------------------------------------------


---- [ui] ui/rfc-2008-non-exhaustive/uninhabited/match_with_exhaustive_patterns.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/match_with_exhaustive_patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/match_with_exhaustive_patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/match_with_exhaustive_patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0004]: non-exhaustive patterns: type `UninhabitedEnum` is non-empty
   |
   |
LL |     match x {} //~ ERROR non-exhaustive patterns
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `UninhabitedEnum`

error[E0004]: non-exhaustive patterns: type `UninhabitedStruct` is non-empty
   |
   |
LL |     match x {} //~ ERROR non-exhaustive patterns
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `UninhabitedStruct`


error[E0004]: non-exhaustive patterns: type `UninhabitedTupleStruct` is non-empty
   |
   |
LL |     match x {} //~ ERROR non-exhaustive patterns
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `UninhabitedTupleStruct`

thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/match_with_exhaustive_patterns.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `cannot_empty_match_on_enum_with_empty_variants_struct_to_anything`
#1 [analysis] running analysis passes on this crate
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0004`.


------------------------------------------


---- [ui] ui/span/transitive-dep-span.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/transitive-dep-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/span/auxiliary/transitive_dep_three.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z macro-backtrace -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/suggestions/attribute-typos.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/attribute-typos.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
   |
   |
LL | #[rustc_err]

error: cannot find attribute `rustc_err` in this scope
  --> /checkout/src/test/ui/suggestions/attribute-typos.rs:7:3
   |
   |
LL | #[rustc_err]
   |   ^^^^^^^^^ help: a built-in attribute with a similar name exists: `rustc_error`

thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/suggestions/attribute-typos.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/suggestions/expected-boxed-future-isnt-pinned.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:11:5
   |
LL | fn foo<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |        - this type parameter                            ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL |     // We could instead use an `async` block, but this way we have no std spans.
LL |     x //~ ERROR mismatched types
   |     |
   |     |
   |     expected struct `Pin`, found type parameter `F`
   |     help: you need to pin and box this expression: `Box::pin(x)`
   |
   = note:      expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
           found type parameter `F`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:18:5
   |
   |
LL | fn bar<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |                                                         ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL |     Box::new(x) //~ ERROR mismatched types
   |     ^^^^^^^^^^^ expected struct `Pin`, found struct `Box`
   |
   = note: expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
              found struct `Box<F>`
   = help: use `Box::pin`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:22:14
   |
   |
LL | fn baz<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |        - this type parameter
LL |     Pin::new(x) //~ ERROR mismatched types
   |              |
   |              |
   |              expected struct `Box`, found type parameter `F`
   |              help: store this in the heap by calling `Box::new`: `Box::new(x)`
   |
   = note:      expected struct `Box<dyn Future<Output = i32> + Send>`
           found type parameter `F`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html

error[E0277]: `dyn Future<Output = i32> + Send` cannot be unpinned
   |
   |
LL |     Pin::new(x) //~ ERROR mismatched types
   |     ^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = i32> + Send`
   |
   = note: consider using `Box::pin`
   = note: required by `Pin::<P>::new`

error[E0277]: `dyn Future<Output = i32> + Send` cannot be unpinned
   |
   |
LL |     Pin::new(Box::new(x)) //~ ERROR E0277
   |     ^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = i32> + Send`
   |
   = note: consider using `Box::pin`
   = note: required by `Pin::<P>::new`

thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `zap`
#1 [typeck_item_bodies] type-checking all item bodies
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/suggestions/imm-ref-trait-object.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/imm-ref-trait-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/suggestions/imm-ref-trait-object.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `test`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
              7: rustc_query_system::query::plumbing::get_query
              8: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path
              9: <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_dyn_existential
             10: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
             11: rustc_middle::ty::print::pretty::<impl rustc_middle::ty::print::Print<P> for rustc_middle::ty::sty::TypeAndMut>::print
             12: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type
             13: rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt
             15: alloc::fmt::format
             15: alloc::fmt::format
             16: rustc_typeck::check::method::suggest::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::report_method_error
             17: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             18: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             19: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             20: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             21: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
             22: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             23: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             24: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
             25: rustc_typeck::check::check::check_fn
             26: rustc_infer::infer::InferCtxtBuilder::enter
             27: rustc_typeck::check::inherited::InheritedBuilder::enter
             28: rustc_typeck::check::typeck
             29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             30: rustc_data_structures::stack::ensure_sufficient_stack
             31: rustc_query_system::query::plumbing::force_query_with_job
             32: rustc_query_system::query::plumbing::get_query_impl
             33: rustc_query_system::query::plumbing::get_query
             34: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
             35: rustc_typeck::check::typeck_item_bodies
             36: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             37: rustc_data_structures::stack::ensure_sufficient_stack
             38: rustc_query_system::query::plumbing::force_query_with_job
             39: rustc_query_system::query::plumbing::get_query_impl
             40: rustc_query_system::query::plumbing::get_query
             41: rustc_session::utils::<impl rustc_session::session::Session>::time
             42: rustc_typeck::check_crate
             43: rustc_interface::passes::analysis
             44: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             45: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             46: rustc_data_structures::stack::ensure_sufficient_stack
             47: rustc_query_system::query::plumbing::force_query_with_job
             48: rustc_query_system::query::plumbing::get_query_impl
             49: rustc_query_system::query::plumbing::get_query
             50: rustc_interface::passes::QueryContext::enter
             51: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             52: rustc_span::with_source_map
             53: scoped_tls::ScopedKey<T>::set
             54: rustc_span::with_session_globals
             56: core::ops::function::FnOnce::call_once{{vtable.shim}}
             57: std::sys::unix::thread::Thread::new::thread_start
             58: start_thread
             59: __clone
             59: __clone
           

thread 'rustc' panicked at 'no warnings or errors encountered even though `delayed_good_path_bugs` issued', compiler/rustc_errors/src/lib.rs:1020:13
stack backtrace:
   0:     0x7f48c79d5ca0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f48c7a465ac - core::fmt::write::hf00778d011964c9e
   2:     0x7f48c79c6036 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f48c79da037 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f48c79d9a44 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f48c82af0ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f48c79da972 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f48c79da4a7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f48c79d613c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f48c79da409 - rust_begin_unwind
  10:     0x7f48c79da3bb - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  11:     0x7f48ca8756d4 - rustc_errors::HandlerInner::flush_delayed::h474f5cea492b1bdc
  12:     0x7f48ca872911 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h05430370eaa235fe
  13:     0x7f48c82bdca2 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h9e51e5c350ff6fdc
  14:     0x7f48c82c199a - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h366bdb50781b3736
  15:     0x7f48c82b3d0c - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h71d7e9c1a290a5fa
  16:     0x7f48c82b1381 - rustc_span::with_source_map::heac904d2395b36aa
  17:     0x7f48c82c64fc - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  18:     0x7f48c82b14f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  19:     0x7f48c82ebb02 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  20:     0x7f48c82ecc3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  21:     0x7f48c79e737a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  22:     0x7f48c269d6db - start_thread
  23:     0x7f48c766971f - __clone
  24:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/suggestions/import-trait-for-method-call.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/import-trait-for-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/import-trait-for-method-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/import-trait-for-method-call/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'annotated_files > 1 in /checkout/src/test/ui/suggestions/import-trait-for-method-call.rs', compiler/rustc_errors/src/emitter.rs:1287:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (57b056042 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `next_u64`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: rustc_errors::Handler::delay_good_path_bug
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::trimmed_def_paths>::compute
              3: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
              4: rustc_data_structures::stack::ensure_sufficient_stack
              5: rustc_query_system::query::plumbing::force_query_with_job
              6: rustc_query_system::query::plumbing::get_query_impl
---
test result: FAILED. 11628 passed; 113 failed; 96 ignored; 0 measured; 0 filtered out; finished in 118.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:09
