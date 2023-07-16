
user@Ondo:~/Code/libm_test$ cat Cargo.toml
[package]
name = "libm_test"
version = "0.1.0"
edition = "2021"

[profile.dev]
lto = "fat"
user@Ondo:~/Code/libm_test$ RUSTC_BOOTSTRAP=1 cargo build --target x86_64-fortanix-unknown-sgx -Zbuild-std
   Compiling libm_test v0.1.0 (/home/user/Code/libm_test)
error: linking with `rust-lld` failed: exit status: 1
  |
  = note: "rust-lld" "-flavor" "gnu" "--version-script=/tmp/rustcuhJyHq/list" "-e" "elf_entry" "-Bstatic" "--gc-sections" "-z" "text" "-z" "norelro" "--no-undefined" "--error-unresolved-symbols" "--no-undefined-version" "-Bsymbolic" "--export-dynamic" "-u" "__rust_abort" "-u" "__rust_c_alloc" "-u" "__rust_c_dealloc" "-u" "__rust_print_err" "-u" "__rust_rwlock_rdlock" "-u" "__rust_rwlock_unlock" "-u" "__rust_rwlock_wrlock" "/tmp/rustcuhJyHq/symbols.o" "/home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libm_test-bc58705d3d7f1468.std-f95466e92e9a6dd6.std.85d6fccd-cgu.0.rcgu.o.rcgu.o" "--as-needed" "-L" "/home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps" "-L" "/home/user/Code/libm_test/target/debug/deps" "-L" "/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-fortanix-unknown-sgx/lib" "-Bstatic" "-lunwind" "/home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib" "-Bdynamic" "--eh-frame-hdr" "-znoexecstack" "-L" "/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-fortanix-unknown-sgx/lib" "-o" "/home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libm_test-bc58705d3d7f1468" "--gc-sections" "-pie"
  = note: rust-lld: error: undefined symbol: core::panicking::panic::h0f3533d9132f92e1
          >>> referenced by rem_pio2f.rs:60 (src/../libm/src/math/rem_pio2f.rs:60)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.1.rcgu.o:(compiler_builtins::math::libm::rem_pio2f::rem_pio2f::h87eccc73280ff7a5) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by rem_pio2f.rs:61 (src/../libm/src/math/rem_pio2f.rs:61)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.1.rcgu.o:(compiler_builtins::math::libm::rem_pio2f::rem_pio2f::h87eccc73280ff7a5) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by rem_pio2f.rs:64 (src/../libm/src/math/rem_pio2f.rs:64)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.1.rcgu.o:(compiler_builtins::math::libm::rem_pio2f::rem_pio2f::h87eccc73280ff7a5) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced 77 more times

          rust-lld: error: undefined symbol: core::panicking::panic_str_nounwind::hbaaec5f04f1a75e3
          >>> referenced by intrinsics.rs:2214 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:2214)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.0.rcgu.o:(core::ptr::read_volatile::h3ac619aa68a75721) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by intrinsics.rs:2214 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:2214)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.0.rcgu.o:(core::ptr::read_volatile::hb8e48ddf5e65cb7b) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by intrinsics.rs:2214 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:2214)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.15.rcgu.o:(_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$::get_unchecked::h67606a0b68744187) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced 4 more times

          rust-lld: error: undefined symbol: core::intrinsics::copy_nonoverlapping::hfb3d5412eadeaedf
          >>> referenced by mod.rs:1232 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1232)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.0.rcgu.o:(core::ptr::read_unaligned::h5feb5085d94c7296) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by mod.rs:1232 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1232)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.0.rcgu.o:(core::ptr::read_unaligned::h84db55d1b209da64) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by mod.rs:1232 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1232)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.0.rcgu.o:(core::ptr::read_unaligned::ha2f349f3b71f94c1) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::ops::range::RangeInclusive$LT$Idx$GT$::new::ha07de38d4900e337
          >>> referenced by rem_pio2_large.rs:260 (src/../libm/src/math/rem_pio2_large.rs:260)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.13.rcgu.o:(compiler_builtins::math::libm::rem_pio2_large::rem_pio2_large::h394954f1322d0319) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by rem_pio2_large.rs:270 (src/../libm/src/math/rem_pio2_large.rs:270)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.13.rcgu.o:(compiler_builtins::math::libm::rem_pio2_large::rem_pio2_large::h394954f1322d0319) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by rem_pio2_large.rs:272 (src/../libm/src/math/rem_pio2_large.rs:272)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.13.rcgu.o:(compiler_builtins::math::libm::rem_pio2_large::rem_pio2_large::h394954f1322d0319) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced 12 more times

          rust-lld: error: undefined symbol: _$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$::into_iter::h231634b2b56c56fb
          >>> referenced by rem_pio2_large.rs:313 (src/../libm/src/math/rem_pio2_large.rs:313)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.13.rcgu.o:(compiler_builtins::math::libm::rem_pio2_large::rem_pio2_large::h394954f1322d0319) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by x86_64.rs:160 (src/mem/x86_64.rs:160)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.4.rcgu.o:(compiler_builtins::mem::impls::compare_bytes::_$u7b$$u7b$closure$u7d$$u7d$::hc4f63bb035933f96) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::iter::range::_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$::next::he9fd8d5fdd4d6791
          >>> referenced by rem_pio2_large.rs:313 (src/../libm/src/math/rem_pio2_large.rs:313)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.13.rcgu.o:(compiler_builtins::math::libm::rem_pio2_large::rem_pio2_large::h394954f1322d0319) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by x86_64.rs:160 (src/mem/x86_64.rs:160)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.4.rcgu.o:(compiler_builtins::mem::impls::compare_bytes::_$u7b$$u7b$closure$u7d$$u7d$::hc4f63bb035933f96) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::cmp::Ord::min::h4dec9fa1a980e168
          >>> referenced by x86_64.rs:179 (src/mem/x86_64.rs:179)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.4.rcgu.o:(compiler_builtins::mem::impls::rep_param::h2029fd4138ed741c) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::panicking::panic_fmt::h550b9c501efa5afd
          >>> referenced by mod.rs:396 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:396)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::fmt::Arguments::new_v1::hfc7532332f1d6c16) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by const_ptr.rs:1358 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1358)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_aligned_to::h2f007e806ab4ecb8) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by const_ptr.rs:1358 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1358)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_aligned_to::hcbfab17ad41c3d7f) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::addr::hb3db00afa0898b5c
          >>> referenced by const_ptr.rs:1362 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1362)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_aligned_to::h2f007e806ab4ecb8) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by const_ptr.rs:1362 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1362)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_aligned_to::hcbfab17ad41c3d7f) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::ptr::read_unaligned::h3a8b67f5025006ab
          >>> referenced by const_ptr.rs:1224 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1224)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::read_unaligned::hcd2700b5ef939424) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::ptr::read::hc11b02ad82c44566
          >>> referenced by const_ptr.rs:1183 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:1183)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::read::h8c6a5842d192e62d) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::ptr::metadata::from_raw_parts::hd64d6a9206a64bc3
          >>> referenced by mod.rs:516 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:516)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_null::h28f840bc152a6d3e) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by mod.rs:516 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:516)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_null::h942b9c261cc14f59) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::guaranteed_eq::hbbdc9adb85c1ccb3
          >>> referenced by const_ptr.rs:39 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:39)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_null::h28f840bc152a6d3e) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by const_ptr.rs:39 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:39)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.8.rcgu.o:(core::ptr::const_ptr::_$LT$impl$u20$$BP$const$u20$T$GT$::is_null::h942b9c261cc14f59) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib

          rust-lld: error: undefined symbol: core::mem::replace::hc497c98da982cdbf
          >>> referenced by range.rs:1057 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:1057)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.9.rcgu.o:(_$LT$core..ops..range..RangeInclusive$LT$T$GT$$u20$as$u20$core..iter..range..RangeInclusiveIteratorImpl$GT$::spec_next_back::h22efa2ea959d5e9f) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib
          >>> referenced by range.rs:1012 (/home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs:1012)
          >>>               compiler_builtins-10d754e8a98988b8.compiler_builtins.e5a09702-cgu.9.rcgu.o:(_$LT$core..ops..range..RangeInclusive$LT$T$GT$$u20$as$u20$core..iter..range..RangeInclusiveIteratorImpl$GT$::spec_next::h784a0f19ab8353e9) in archive /home/user/Code/libm_test/target/x86_64-fortanix-unknown-sgx/debug/deps/libcompiler_builtins-10d754e8a98988b8.rlib


error: could not compile `libm_test` due to previous error
user@Ondo:~/Code/libm_test$
