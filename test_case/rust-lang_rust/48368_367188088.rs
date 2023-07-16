
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0 (4d90ac38c 2018-02-12) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 18446744073709551615', /checkout/src/liballoc/vec.rs:1551:10
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at /checkout/src/libstd/sys_common/backtrace.rs:68
             at /checkout/src/libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: rustdoc::html::render::get_index_type_name
             at /checkout/src/liballoc/vec.rs:1551
             at /checkout/src/librustdoc/html/render.rs:4139
  11: rustdoc::html::render::get_index_type
             at /checkout/src/librustdoc/html/render.rs:4129
  12: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at /checkout/src/librustdoc/html/render.rs:4118
             at /checkout/src/libcore/ops/function.rs:271
             at /checkout/src/libcore/option.rs:404
             at /checkout/src/libcore/iter/mod.rs:1251
             at /checkout/src/liballoc/vec.rs:1844
             at /checkout/src/liballoc/vec.rs:1827
  13: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
             at /checkout/src/liballoc/vec.rs:1713
             at /checkout/src/libcore/iter/iterator.rs:1298
             at /checkout/src/librustdoc/html/render.rs:4118
             at /checkout/src/librustdoc/html/render.rs:1278
  14: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at /checkout/src/librustdoc/fold.rs:71
             at /checkout/src/libcore/iter/mod.rs:1497
             at /checkout/src/liballoc/vec.rs:1921
             at /checkout/src/liballoc/vec.rs:1818
             at /checkout/src/liballoc/vec.rs:1813
  15: rustdoc::fold::DocFolder::fold_inner_recur
             at /checkout/src/liballoc/vec.rs:1713
             at /checkout/src/libcore/iter/iterator.rs:1298
             at /checkout/src/librustdoc/fold.rs:71
  16: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
             at /checkout/src/librustdoc/fold.rs:97
             at /checkout/src/librustdoc/html/render.rs:1373
  17: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at /checkout/src/librustdoc/fold.rs:107
             at /checkout/src/libcore/iter/mod.rs:1497
             at /checkout/src/liballoc/vec.rs:1921
             at /checkout/src/liballoc/vec.rs:1818
             at /checkout/src/liballoc/vec.rs:1813
  18: rustdoc::fold::DocFolder::fold_inner_recur
             at /checkout/src/liballoc/vec.rs:1713
             at /checkout/src/libcore/iter/iterator.rs:1298
             at /checkout/src/librustdoc/fold.rs:107
             at /checkout/src/librustdoc/fold.rs:43
  19: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
             at /checkout/src/librustdoc/fold.rs:97
             at /checkout/src/librustdoc/html/render.rs:1373
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at /checkout/src/librustdoc/fold.rs:107
             at /checkout/src/libcore/iter/mod.rs:1497
             at /checkout/src/liballoc/vec.rs:1921
             at /checkout/src/liballoc/vec.rs:1818
             at /checkout/src/liballoc/vec.rs:1813
  21: rustdoc::fold::DocFolder::fold_inner_recur
             at /checkout/src/liballoc/vec.rs:1713
             at /checkout/src/libcore/iter/iterator.rs:1298
             at /checkout/src/librustdoc/fold.rs:107
             at /checkout/src/librustdoc/fold.rs:43
  22: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
             at /checkout/src/librustdoc/fold.rs:97
             at /checkout/src/librustdoc/html/render.rs:1373
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at /checkout/src/librustdoc/fold.rs:107
             at /checkout/src/libcore/iter/mod.rs:1497
             at /checkout/src/liballoc/vec.rs:1921
             at /checkout/src/liballoc/vec.rs:1818
             at /checkout/src/liballoc/vec.rs:1813
  24: rustdoc::fold::DocFolder::fold_inner_recur
             at /checkout/src/liballoc/vec.rs:1713
             at /checkout/src/libcore/iter/iterator.rs:1298
             at /checkout/src/librustdoc/fold.rs:107
             at /checkout/src/librustdoc/fold.rs:43
  25: <rustdoc::html::render::Cache as rustdoc::fold::DocFolder>::fold_item
             at /checkout/src/librustdoc/fold.rs:97
             at /checkout/src/librustdoc/html/render.rs:1373
  26: rustdoc::html::render::run
             at /checkout/src/librustdoc/fold.rs:112
             at /checkout/src/libcore/option.rs:616
             at /checkout/src/librustdoc/fold.rs:112
             at /checkout/src/librustdoc/html/render.rs:637

error: Could not document `rspotify`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name rspotify src/lib.rs -o /home/wmoore/Source/rspotify/target/doc -L dependency=/home/wmoore/Source/rspotify/target/debug/deps --extern itertools=/home/wmoore/Source/rspotify/target/debug/deps/libitertools-a00472a3fc42da71.rlib --extern chrono=/home/wmoore/Source/rspotify/target/debug/deps/libchrono-d2f3d51acae581db.rlib --extern rand=/home/wmoore/Source/rspotify/target/debug/deps/librand-5c6187fa023d4e5b.rlib --extern derive_builder=/home/wmoore/Source/rspotify/target/debug/deps/libderive_builder-17c9d53b864b117b.so --extern error_chain=/home/wmoore/Source/rspotify/target/debug/deps/liberror_chain-8e65f83670303dec.rlib --extern hyper=/home/wmoore/Source/rspotify/target/debug/deps/libhyper-2fdd5afd82d5c1d2.rlib --extern dotenv=/home/wmoore/Source/rspotify/target/debug/deps/libdotenv-2e1ad2409a05b6ff.rlib --extern log=/home/wmoore/Source/rspotify/target/debug/deps/liblog-e57f6942844e1687.rlib --extern serde=/home/wmoore/Source/rspotify/target/debug/deps/libserde-b719a0583d8b0c97.rlib --extern webbrowser=/home/wmoore/Source/rspotify/target/debug/deps/libwebbrowser-1891442121321d7e.rlib --extern env_logger=/home/wmoore/Source/rspotify/target/debug/deps/libenv_logger-189d105f596d50af.rlib --extern base64=/home/wmoore/Source/rspotify/target/debug/deps/libbase64-fc9b6d117fb30580.rlib --extern url=/home/wmoore/Source/rspotify/target/debug/deps/liburl-4452dd177b1b968c.rlib --extern random=/home/wmoore/Source/rspotify/target/debug/deps/librandom-230e99222d408c2f.rlib --extern serde_json=/home/wmoore/Source/rspotify/target/debug/deps/libserde_json-c338d2b9d0b76720.rlib --extern percent_encoding=/home/wmoore/Source/rspotify/target/debug/deps/libpercent_encoding-1b677aae1c9b78d4.rlib --extern serde_derive=/home/wmoore/Source/rspotify/target/debug/deps/libserde_derive-62ba8cf53e26a7fd.so --extern reqwest=/home/wmoore/Source/rspotify/target/debug/deps/libreqwest-f89f0f5ac696c386.rlib --extern lazy_static=/home/wmoore/Source/rspotify/target/debug/deps/liblazy_static-6e5677b59a5901c1.rlib` (exit code: 101)
