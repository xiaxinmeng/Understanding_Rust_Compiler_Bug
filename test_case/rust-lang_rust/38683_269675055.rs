
test hamt::tests::stress_test_copy ... thread 'hamt::tests::stress_test_copy' panicked at 'assertion failed: new_i == (new_node.entry_count() as usize)', src/hamt.rs:985
stack backtrace:
   1:     0x5594987f996a - std::sys::imp::backtrace::tracing::imp::write::hc924c01e14fdf084
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x5594987fcb0f - std::panicking::default_hook::{{closure}}::h3c7e4ff009c61db6
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:349
   3:     0x5594987fc70e - std::panicking::default_hook::h4684c234c8ae64c1
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:365
   4:     0x5594987fcfb7 - std::panicking::rust_panic_with_hook::h03d5a3ece8e80bc6
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:553
   5:     0x55949876fe23 - std::panicking::begin_panic::ha0a1db1d8e138dd3
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:515
   6:     0x559498791857 - <hamt_rs::hamt::UnsafeNode<K, V, IS, H>>::copy_with_new_entry::h6c170073efd5c707
                        at /home/alex/code/hamt-rs/src/hamt.rs:985
   7:     0x5594987891a5 - <hamt_rs::hamt::UnsafeNode<K, V, IS, H>>::insert::ha7423a2a61f77503
                        at /home/alex/code/hamt-rs/src/hamt.rs:445
   8:     0x55949878a3fa - <hamt_rs::hamt::UnsafeNode<K, V, IS, H>>::try_insert_in_place::h1bcf3ca8f1746725
                        at /home/alex/code/hamt-rs/src/hamt.rs:559
   9:     0x559498795caf - <hamt_rs::hamt::HamtMap<K, V, IS, H>>::insert_internal::h537ed4c9d168ec7b
                        at /home/alex/code/hamt-rs/src/hamt.rs:1233
  10:     0x559498797266 - <hamt_rs::hamt::HamtMap<K, V, IS, H>>::insert::hd0643ef38ae5494f
                        at /home/alex/code/hamt-rs/src/hamt.rs:1310
  11:     0x5594987a7da7 - hamt_rs::testing::Test::random_insert_remove_stress_test::he6910fa41eadaaf1
                        at /home/alex/code/hamt-rs/src/testing.rs:211
  12:     0x55949879afb9 - hamt_rs::hamt::tests::stress_test_copy::hf915b13afc66be76
                        at /home/alex/code/hamt-rs/src/hamt.rs:1699
  13:     0x5594987cae1e - <F as test::FnBox<T>>::call_box::h5eb8a1547b6faa57
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libtest/lib.rs:1349
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libtest/lib.rs:140
  14:     0x559498803eca - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  15:     0x5594987bf293 - std::panicking::try::do_call::h22cb7a85a3940175
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:434
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libtest/lib.rs:1294
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panic.rs:295
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:458
  16:     0x559498803eca - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  17:     0x5594987c5e62 - <F as alloc::boxed::FnBox<A>>::call_box::h676aac362446d380
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panicking.rs:434
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/thread/mod.rs:301
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/liballoc/boxed.rs:605
  18:     0x5594987fbe14 - std::sys::imp::thread::Thread::new::thread_start::h29f4c99db4339ba7
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/liballoc/boxed.rs:615
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  19:     0x7f38cd5236b9 - start_thread
  20:     0x7f38cd04382c - clone
21: 0x0 - <unknown>
