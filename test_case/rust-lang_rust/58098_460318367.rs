plain
travis_time:end:12b9b575:start=1549293098169034197,finish=1549293169766995051,duration=71597960854
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:49]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:24:49]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:24:49]    Compiling rustc-demangle v0.1.10
[00:24:55]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:24:56] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:24:56]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:24:56] 82 | #[allow_internal_unstable]
[00:24:56]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:24:56] 
[00:25:15]     Finished release [optimized] target(s) in 1m 08s
---
[00:26:20]    Compiling flate2 v1.0.6
[00:26:21]    Compiling parking_lot v0.6.4
[00:26:21]    Compiling rustc-rayon v0.1.1
[00:26:25]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:26:26] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:26]   |
[00:26:26] 4 | #[allow_internal_unstable]
[00:26:26]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:26] 
[00:26:26] 
[00:26:27]    Compiling tempfile v3.0.5
[00:26:30]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:26:30]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:26:30] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:30]   --> /checkout/src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:26:30] 82 | #[allow_internal_unstable]
[00:26:30]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:30] 
[00:26:30] 
[00:26:30] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:30]    |
[00:26:30] 71 | #[allow_internal_unstable]
[00:26:30]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:30] 
[00:26:30] 
[00:26:36]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:26:52] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:52]    |
[00:26:52] 71 | #[allow_internal_unstable]
[00:26:52]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:52] 
[00:26:52] 
[00:26:52] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:52]   |
[00:26:52] 4 | #[allow_internal_unstable]
[00:26:52]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:52] 
[00:26:52] 
[00:28:07]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:28:09] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:28:09]   |
[00:28:09] 4 | #[allow_internal_unstable]
[00:28:09]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:28:09] 
[00:28:09] 
[00:28:10] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:28:10]    |
[00:28:10] 71 | #[allow_internal_unstable]
[00:28:10]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:28:10] 
---
[00:57:26]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:57:26]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:57:26]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:57:26]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:57:27] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:57:27]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:57:27] 82 | #[allow_internal_unstable]
[00:57:27]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:57:27] 
[00:57:38]     Finished release [optimized] target(s) in 18.44s
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:39] 
[01:09:39] running 119 tests
[01:10:04] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:08] i......iii.i.....ii
[01:10:08] 
[01:10:08]  finished in 29.047
[01:10:08] travis_fold:end:test_debuginfo

---
[01:25:58] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:59]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:25:59] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[01:25:59]   --> /checkout/src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[01:25:59] 82 | #[allow_internal_unstable]
[01:25:59]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:25:59] 
[01:25:59] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::smoke': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03]     |
[01:26:03] 380 | /     fn smoke() {
[01:26:03] 381 | |         let (tx1, rx1) = channel::<i32>();
[01:26:03] 382 | |         let (tx2, rx2) = channel::<i32>();
[01:26:03] 383 | |         tx1.send(1).unwrap();
[01:26:03] 401 | |         }
[01:26:03] 402 | |     }
[01:26:03]     | |_____^
[01:26:03]     |
[01:26:03]     |
[01:26:03]     = note: `-D deprecated` implied by `-D warnings`
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::smoke2': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03]     |
[01:26:03] 405 | /     fn smoke2() {
[01:26:03] 406 | |         let (_tx1, rx1) = channel::<i32>();
[01:26:03] 407 | |         let (_tx2, rx2) = channel::<i32>();
[01:26:03] 408 | |         let (_tx3, rx3) = channel::<i32>();
[01:26:03] 418 | |         }
[01:26:03] 419 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::closed': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 422 | /     fn closed() {
[01:26:03] 422 | /     fn closed() {
[01:26:03] 423 | |         let (_tx1, rx1) = channel::<i32>();
[01:26:03] 424 | |         let (tx2, rx2) = channel::<i32>();
[01:26:03] 425 | |         drop(tx2);
[01:26:03] 430 | |         }
[01:26:03] 431 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::unblocks': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 434 | /     fn unblocks() {
[01:26:03] 434 | /     fn unblocks() {
[01:26:03] 435 | |         let (tx1, rx1) = channel::<i32>();
[01:26:03] 436 | |         let (_tx2, rx2) = channel::<i32>();
[01:26:03] 437 | |         let (tx3, rx3) = channel::<i32>();
[01:26:03] 454 | |         }
[01:26:03] 455 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::both_ready': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 458 | /     fn both_ready() {
[01:26:03] 458 | /     fn both_ready() {
[01:26:03] 459 | |         let (tx1, rx1) = channel::<i32>();
[01:26:03] 460 | |         let (tx2, rx2) = channel::<i32>();
[01:26:03] 461 | |         let (tx3, rx3) = channel::<()>();
[01:26:03] ...   |
[01:26:03] 480 | |         tx3.send(()).unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::stress': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 484 | /     fn stress() {
[01:26:03] 484 | /     fn stress() {
[01:26:03] 485 | |         const AMT: i32 = 10000;
[01:26:03] 486 | |         let (tx1, rx1) = channel::<i32>();
[01:26:03] 487 | |         let (tx2, rx2) = channel::<i32>();
[01:26:03] 507 | |         }
[01:26:03] 508 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::cloning': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 512 | /     fn cloning() {
[01:26:03] 512 | /     fn cloning() {
[01:26:03] 513 | |         let (tx1, rx1) = channel::<i32>();
[01:26:03] 514 | |         let (_tx2, rx2) = channel::<i32>();
[01:26:03] 515 | |         let (tx3, rx3) = channel::<()>();
[01:26:03] ...   |
[01:26:03] 530 | |         tx3.send(()).unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::cloning2': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 535 | /     fn cloning2() {
[01:26:03] 535 | /     fn cloning2() {
[01:26:03] 536 | |         let (tx1, rx1) = channel::<i32>();
[01:26:03] 537 | |         let (_tx2, rx2) = channel::<i32>();
[01:26:03] 538 | |         let (tx3, rx3) = channel::<()>();
[01:26:03] ...   |
[01:26:03] 553 | |         tx3.send(()).unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::cloning3': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 557 | /     fn cloning3() {
[01:26:03] 557 | /     fn cloning3() {
[01:26:03] 558 | |         let (tx1, rx1) = channel::<()>();
[01:26:03] 559 | |         let (tx2, rx2) = channel::<()>();
[01:26:03] 560 | |         let (tx3, rx3) = channel::<()>();
[01:26:03] ...   |
[01:26:03] 574 | |         rx3.recv().unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight1': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 578 | /     fn preflight1() {
[01:26:03] 578 | /     fn preflight1() {
[01:26:03] 579 | |         let (tx, rx) = channel();
[01:26:03] 580 | |         tx.send(()).unwrap();
[01:26:03] 581 | |         select! {
[01:26:03] 582 | |             _n = rx.recv() => {}
[01:26:03] 584 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight2': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 587 | /     fn preflight2() {
[01:26:03] 587 | /     fn preflight2() {
[01:26:03] 588 | |         let (tx, rx) = channel();
[01:26:03] 589 | |         tx.send(()).unwrap();
[01:26:03] 590 | |         tx.send(()).unwrap();
[01:26:03] 593 | |         }
[01:26:03] 594 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight3': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 597 | /     fn preflight3() {
[01:26:03] 597 | /     fn preflight3() {
[01:26:03] 598 | |         let (tx, rx) = channel();
[01:26:03] 599 | |         drop(tx.clone());
[01:26:03] 600 | |         tx.send(()).unwrap();
[01:26:03] 603 | |         }
[01:26:03] 604 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight4': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 607 | /     fn preflight4() {
[01:26:03] 607 | /     fn preflight4() {
[01:26:03] 608 | |         let (tx, rx) = channel();
[01:26:03] 609 | |         tx.send(()).unwrap();
[01:26:03] 610 | |         let s = Select::new();
[01:26:03] ...   |
[01:26:03] 613 | |         assert_eq!(s.wait2(false), h.id);
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight5': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 617 | /     fn preflight5() {
[01:26:03] 617 | /     fn preflight5() {
[01:26:03] 618 | |         let (tx, rx) = channel();
[01:26:03] 619 | |         tx.send(()).unwrap();
[01:26:03] 620 | |         tx.send(()).unwrap();
[01:26:03] ...   |
[01:26:03] 624 | |         assert_eq!(s.wait2(false), h.id);
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight6': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 628 | /     fn preflight6() {
[01:26:03] 628 | /     fn preflight6() {
[01:26:03] 629 | |         let (tx, rx) = channel();
[01:26:03] 630 | |         drop(tx.clone());
[01:26:03] 631 | |         tx.send(()).unwrap();
[01:26:03] ...   |
[01:26:03] 635 | |         assert_eq!(s.wait2(false), h.id);
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight7': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 639 | /     fn preflight7() {
[01:26:03] 639 | /     fn preflight7() {
[01:26:03] 640 | |         let (tx, rx) = channel::<()>();
[01:26:03] 641 | |         drop(tx);
[01:26:03] 642 | |         let s = Select::new();
[01:26:03] ...   |
[01:26:03] 645 | |         assert_eq!(s.wait2(false), h.id);
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight8': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 649 | /     fn preflight8() {
[01:26:03] 649 | /     fn preflight8() {
[01:26:03] 650 | |         let (tx, rx) = channel();
[01:26:03] 651 | |         tx.send(()).unwrap();
[01:26:03] 652 | |         drop(tx);
[01:26:03] ...   |
[01:26:03] 657 | |         assert_eq!(s.wait2(false), h.id);
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::preflight9': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 661 | /     fn preflight9() {
[01:26:03] 661 | /     fn preflight9() {
[01:26:03] 662 | |         let (tx, rx) = channel();
[01:26:03] 663 | |         drop(tx.clone());
[01:26:03] 664 | |         tx.send(()).unwrap();
[01:26:03] ...   |
[01:26:03] 670 | |         assert_eq!(s.wait2(false), h.id);
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::oneshot_data_waiting': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 674 | /     fn oneshot_data_waiting() {
[01:26:03] 674 | /     fn oneshot_data_waiting() {
[01:26:03] 675 | |         let (tx1, rx1) = channel();
[01:26:03] 676 | |         let (tx2, rx2) = channel();
[01:26:03] 677 | |         let _t = thread::spawn(move|| {
[01:26:03] ...   |
[01:26:03] 686 | |         rx2.recv().unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::stream_data_waiting': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03] 690 | /     fn stream_data_waiting() {
[01:26:03] 690 | /     fn stream_data_waiting() {
[01:26:03] 691 | |         let (tx1, rx1) = channel();
[01:26:03] 692 | |         let (tx2, rx2) = channel();
[01:26:03] 693 | |         tx1.send(()).unwrap();
[01:26:03] ...   |
[01:26:03] 706 | |         rx2.recv().unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::shared_data_waiting': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03]     |
[01:26:03] 710 | /     fn shared_data_waiting() {
[01:26:03] 711 | |         let (tx1, rx1) = channel();
[01:26:03] 712 | |         let (tx2, rx2) = channel();
[01:26:03] 713 | |         drop(tx1.clone());
[01:26:03] ...   |
[01:26:03] 725 | |         rx2.recv().unwrap();
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::sync1': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03]     |
[01:26:03] 729 | /     fn sync1() {
[01:26:03] 730 | |         let (tx, rx) = sync_channel::<i32>(1);
[01:26:03] 731 | |         tx.send(1).unwrap();
[01:26:03] 732 | |         select! {
[01:26:03] 733 | |             n = rx.recv() => { assert_eq!(n.unwrap(), 1); }
[01:26:03] 735 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::sync2': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03]     |
[01:26:03] 738 | /     fn sync2() {
[01:26:03] 739 | |         let (tx, rx) = sync_channel::<i32>(0);
[01:26:03] 740 | |         let _t = thread::spawn(move|| {
[01:26:03] 741 | |             for _ in 0..100 { thread::yield_now() }
[01:26:03] 746 | |         }
[01:26:03] 747 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:03] error: use of deprecated item 'sync::mpsc::select::tests::sync3': channel selection will be removed in a future release
[01:26:03]     |
[01:26:03]     |
[01:26:03] 750 | /     fn sync3() {
[01:26:03] 751 | |         let (tx1, rx1) = sync_channel::<i32>(0);
[01:26:03] 752 | |         let (tx2, rx2): (Sender<i32>, Receiver<i32>) = channel();
[01:26:03] 753 | |         let _t = thread::spawn(move|| { tx1.send(1).unwrap(); });
[01:26:03] 766 | |         }
[01:26:03] 767 | |     }
[01:26:03]     | |_____^
[01:26:03] 
[01:26:03] 
[01:26:11] error: aborting due to 24 previous errors
[01:26:11] 
[01:26:12] error: Could not compile `std`.
[01:26:12] 
[01:26:12] To learn more, run the command again with --verbose.
[01:26:12] 
[01:26:12] 
[01:26:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:26:12] 
[01:26:12] 
[01:26:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:12] Build completed unsuccessfully in 0:28:14
[01:26:12] Build completed unsuccessfully in 0:28:14
[01:26:12] make: *** [check] Error 1
[01:26:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bc2669a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 16:39:21 UTC 2019
