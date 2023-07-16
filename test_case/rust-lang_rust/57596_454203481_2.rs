rust
xftroxgpx@z5 2019/01/15 00:44:21 -bash5.0.0 t:10 j:0 d:3 pp:895 p:9093 ut22682
!79538 8 1  5.0.0-rc1-gbfeffd155283 #33 SMP PREEMPT Tue Jan 8 00:42:45 CET 2019
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust 
$ ./clean 
    Finished dev [unoptimized] target(s) in 0.95s
./go
Build completed successfully in 0:00:07
-----------
xftroxgpx@z5 2019/01/15 00:47:47 -bash5.0.0 t:10 j:0 d:3 pp:895 p:9093 ut22887
!79539 9 0  5.0.0-rc1-gbfeffd155283 #33 SMP PREEMPT Tue Jan 8 00:42:45 CET 2019
/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust 
$ ./go
~/build/2nonpkgs/rust.stuff/rust/rust ~/build/2nonpkgs/rust.stuff/rust
./go:47+ set -xev
#export RUSTFLAGS="-Z verbose"
export RUSTFLAGS="-Zverbose"
./go:49+ export RUSTFLAGS=-Zverbose
./go:49+ RUSTFLAGS=-Zverbose
#verbose='-vv'
verbose=''
./go:51+ verbose=
threads=5
./go:52+ threads=5
#threads=1
time PATH="${HOME}/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:${PATH}" python2 ./x.py build $verbose --incremental -j $threads
./go:54+ PATH=/home/xftroxgpx/build/1packages/4used/chro/4_chromium-dev-git/makepkg_pacman/chromium-dev/src/python-path/:/opt/google-cloud-sdk/bin:/opt/google-cloud-sdk/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin:/opt/depot_tools:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl:/home/xftroxgpx/bin:/home/xftroxgpx/build/2nonpkgs/rust.stuff/racer/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/debug:/home/xftroxgpx/build/2nonpkgs/rust.stuff/chars/target/release:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rustfmt/target/release:/home/xftroxgpx/.cargo/bin
./go:54+ python2 ./x.py build --incremental -j 5
extracting /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/cache/2019-01-04/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
extracting /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/cache/2019-01-04/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
extracting /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
    Finished dev [unoptimized] target(s) in 0.66s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.25
   Compiling core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
   Compiling libc v0.2.46
   Compiling build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
   Compiling unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
   Compiling compiler_builtins v0.1.4
   Compiling cmake v0.1.33
   Compiling backtrace-sys v0.1.27
   Compiling std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
   Compiling rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
   Compiling rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
   Compiling rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
   Compiling rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
warning: unnecessary `unsafe` block===========>                        ] 21/36
   --> src/libcore/num/mod.rs:71:30
    |
71  |                           Some(unsafe { $Ty(n) })
    |                                ^^^^^^ unnecessary `unsafe` block
...
100 | / nonzero_integers! {
101 | |     NonZeroU8(u8);
102 | |     NonZeroU16(u16);
103 | |     NonZeroU32(u32);
...   |
106 | |     NonZeroUsize(usize);
107 | | }
    | |_- in this macro invocation
    |
    = note: #[warn(unused_unsafe)] on by default

warning: unnecessary `unsafe` block
   --> src/libcore/num/mod.rs:71:30
    |
71  |                           Some(unsafe { $Ty(n) })
    |                                ^^^^^^ unnecessary `unsafe` block
...
100 | / nonzero_integers! {
101 | |     NonZeroU8(u8);
102 | |     NonZeroU16(u16);
103 | |     NonZeroU32(u32);
...   |
106 | |     NonZeroUsize(usize);
107 | | }
    | |_- in this macro invocation

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:2786:18
     |
2786 |             Some(unsafe { Unique { pointer: ptr as _, _marker: PhantomData } })
     |                  ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:2842:9
     |
2842 |         unsafe { Unique { pointer: reference as *mut T, _marker: PhantomData } }
     |         ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:2849:9
     |
2849 |         unsafe { Unique { pointer: reference as *const T, _marker: PhantomData } }
     |         ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:2856:9
     |
2856 |         unsafe { Unique { pointer: p.pointer, _marker: PhantomData } }
     |         ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:3045:9
     |
3045 |         unsafe { NonNull { pointer: unique.pointer } }
     |         ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:3053:9
     |
3053 |         unsafe { NonNull { pointer: reference as *mut T } }
     |         ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
    --> src/libcore/ptr.rs:3061:9
     |
3061 |         unsafe { NonNull { pointer: reference as *const T } }
     |         ^^^^^^ unnecessary `unsafe` block

warning: unused attribute
   --> src/libcore/num/mod.rs:50:17
    |
50  |                   #[rustc_layout_scalar_valid_range_start(1)]
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
100 | / nonzero_integers! {
101 | |     NonZeroU8(u8);
102 | |     NonZeroU16(u16);
103 | |     NonZeroU32(u32);
...   |
106 | |     NonZeroUsize(usize);
107 | | }
    | |_- in this macro invocation
    |
    = note: #[warn(unused_attributes)] on by default

warning: unused attribute
   --> src/libcore/num/mod.rs:50:17
    |
50  |                   #[rustc_layout_scalar_valid_range_start(1)]
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
100 | / nonzero_integers! {
101 | |     NonZeroU8(u8);
102 | |     NonZeroU16(u16);
103 | |     NonZeroU32(u32);
...   |
106 | |     NonZeroUsize(usize);
107 | | }
    | |_- in this macro invocation

warning: unused attribute
    --> src/libcore/ptr.rs:2721:1
     |
2721 | #[rustc_layout_scalar_valid_range_start(1)]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

[RUSTC-TIMING] core test:false 76.515
   Compiling rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.088               ] 22/36
[RUSTC-TIMING] libc test:false 2.532=============>                     ] 23/36
[RUSTC-TIMING] compiler_builtins test:false 3.592==>                   ] 24/36
   Compiling alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
   Compiling rustc-demangle v0.1.10
   Compiling panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
[RUSTC-TIMING] backtrace_sys test:false 0.170=======>                  ] 25/36
[RUSTC-TIMING] panic_abort test:false 0.198===========>                ] 26/36
[RUSTC-TIMING] unwind test:false 0.280=================>               ] 27/36
[RUSTC-TIMING] rustc_demangle test:false 1.431===========>             ] 28/36
[RUSTC-TIMING] alloc test:false 11.056====================>            ] 29/36
   Compiling panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
[RUSTC-TIMING] rustc_tsan test:false 0.109==================>          ] 30/36
[RUSTC-TIMING] rustc_asan test:false 0.113====================>        ] 31/36
[RUSTC-TIMING] rustc_lsan test:false 0.124=====================>       ] 32/36
[RUSTC-TIMING] rustc_msan test:false 0.132=======================>     ] 33/36
[RUSTC-TIMING] panic_unwind test:false 0.487======================>    ] 34/36
[RUSTC-TIMING] std test:false 21.205================================>  ] 35/36
    Finished release [optimized] target(s) in 1m 54s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 114.338
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
   Compiling getopts v0.2.17
   Compiling proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
[RUSTC-TIMING] getopts test:false 3.918                                  ] 0/4
[RUSTC-TIMING] term test:false 4.585                                     ] 1/4
[RUSTC-TIMING] proc_macro test:false 13.086                              ] 2/4
   Compiling test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
[RUSTC-TIMING] test test:false 6.955=====================>               ] 3/4
    Finished release [optimized] target(s) in 20.88s
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 20.918
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cfg-if v0.1.6
   Compiling libc v0.2.46
   Compiling nodrop v0.1.12
   Compiling rand_core v0.3.0
   Compiling scopeguard v0.3.3
[RUSTC-TIMING] cfg_if test:false 0.105                                 ] 0/128
   Compiling memoffset v0.2.1
[RUSTC-TIMING] memoffset test:false 0.133                              ] 1/128
   Compiling lazy_static v1.2.0
[RUSTC-TIMING] scopeguard test:false 0.228                             ] 2/128
   Compiling void v1.0.2
[RUSTC-TIMING] nodrop test:false 0.248                                 ] 3/128
   Compiling stable_deref_trait v1.1.0
[RUSTC-TIMING] void test:false 0.183                                   ] 4/128
[RUSTC-TIMING] stable_deref_trait test:false 0.189                     ] 5/128
[RUSTC-TIMING] lazy_static test:false 0.339                            ] 6/128
   Compiling rustc-rayon-core v0.1.1
   Compiling semver-parser v0.7.0
   Compiling bitflags v1.0.4
[RUSTC-TIMING] bitflags test:false 0.176                               ] 7/128
[RUSTC-TIMING] rand_core test:false 1.086                              ] 8/128
   Compiling unicode-width v0.1.5
   Compiling byteorder v1.2.7
   Compiling either v1.5.0
   Compiling graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
[RUSTC-TIMING] unicode_width test:false 0.350                         ] 11/128
   Compiling cc v1.0.25
[RUSTC-TIMING] either test:false 0.587                                ] 12/128
   Compiling scoped-tls v0.1.2
[RUSTC-TIMING] scoped_tls test:false 0.525                            ] 13/128
   Compiling crc32fast v1.1.2
[RUSTC-TIMING] byteorder test:false 1.875                             ] 14/128
[RUSTC-TIMING] graphviz test:false 2.107                              ] 15/128
   Compiling rustc_target v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_target)
   Compiling lazy_static v0.2.11
   Compiling termcolor v1.0.4
   Compiling syntax v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax)
[RUSTC-TIMING] lazy_static test:false 0.335                           ] 18/128
   Compiling datafrog v2.0.1
   Compiling rustc-demangle v0.1.10
   Compiling rustc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
   Compiling remove_dir_all v0.5.1
[RUSTC-TIMING] datafrog test:false 1.996                              ] 22/128
   Compiling rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
[RUSTC-TIMING] remove_dir_all test:false 0.211                        ] 23/128
   Compiling fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
[RUSTC-TIMING] rustc_demangle test:false 2.078                        ] 24/128
   Compiling rustc_metadata v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
[RUSTC-TIMING] termcolor test:false 3.034                             ] 25/128
   Compiling rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
   Compiling rustc-serialize v0.3.24
[RUSTC-TIMING] rustc_fs_util test:false 1.147                         ] 27/128
   Compiling rustc_platform_intrinsics v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_platform_intrinsics)
   Compiling quick-error v1.2.2
[RUSTC-TIMING] quick_error test:false 0.264                           ] 29/128
   Compiling rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
[RUSTC-TIMING] fmt_macros test:false 3.351                            ] 31/128
   Compiling crossbeam-utils v0.2.2
[RUSTC-TIMING] crossbeam_utils test:false 0.828                       ] 32/128
   Compiling log v0.4.6
[RUSTC-TIMING] log test:false 1.444                                   ] 33/128
   Compiling arrayvec v0.4.7
[RUSTC-TIMING] arrayvec test:false 1.938                              ] 34/128
   Compiling unreachable v1.0.0
[RUSTC-TIMING] unreachable test:false 0.162                           ] 35/128
   Compiling owning_ref v0.3.3
[RUSTC-TIMING] owning_ref test:false 0.979                            ] 36/128
   Compiling log_settings v0.1.2
[RUSTC-TIMING] log_settings test:false 0.673                          ] 37/128
   Compiling rand_core v0.2.2
[RUSTC-TIMING] rand_core test:false 0.191                             ] 38/128
   Compiling rand_hc v0.1.0
[RUSTC-TIMING] rand_hc test:false 1.642                               ] 39/128
   Compiling rand_isaac v0.1.1
[RUSTC-TIMING] rand_isaac test:false 1.785                            ] 40/128
   Compiling rand_xorshift v0.1.0
   Compiling rustc-hash v1.0.1
[RUSTC-TIMING] rustc_hash test:false 0.436                            ] 44/128
   Compiling semver v0.9.0
[RUSTC-TIMING] rand_xorshift test:false 0.749                         ] 46/128
   Compiling chalk-macros v0.1.0
[RUSTC-TIMING] chalk_macros test:false 1.837                          ] 47/128
   Compiling humantime v1.2.0
[RUSTC-TIMING] rustc_serialize test:false 19.421                      ] 53/128
[RUSTC-TIMING] cc test:false 17.294=>                                 ] 54/128
   Compiling ena v0.11.0
   Compiling rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
   Compiling crossbeam-epoch v0.3.1
[RUSTC-TIMING] humantime test:false 2.727                             ] 57/128
   Compiling smallvec v0.6.7
[RUSTC-TIMING] rustc_cratesio_shim test:false 0.628                   ] 58/128
   Compiling lock_api v0.1.3
[RUSTC-TIMING] ena test:false 1.144===>                               ] 59/128
   Compiling backtrace-sys v0.1.27
[RUSTC-TIMING] smallvec test:false 1.714                              ] 60/128
[RUSTC-TIMING] lock_api test:false 1.180                              ] 61/128
   Compiling miniz-sys v0.1.11
[RUSTC-TIMING] crossbeam_epoch test:false 3.190                       ] 62/128
   Compiling polonius-engine v0.6.2
   Compiling chalk-engine v0.9.0
[RUSTC-TIMING] rustc_platform_intrinsics test:false 25.049            ] 65/128
   Compiling rls-span v0.4.0
[RUSTC-TIMING] crc32fast test:false 1.398>                            ] 66/128
   Compiling rustc_version v0.2.3
[RUSTC-TIMING] polonius_engine test:false 2.480                       ] 67/128
   Compiling serialize v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
[RUSTC-TIMING] rls_span test:false 1.932==>                           ] 68/128
   Compiling rustc_apfloat v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
[RUSTC-TIMING] libc test:false 3.879=======>                          ] 69/128
   Compiling crossbeam-deque v0.2.0
[RUSTC-TIMING] crossbeam_deque test:false 0.683                       ] 70/128
[RUSTC-TIMING] rustc_apfloat test:false 5.971                         ] 72/128
   Compiling rls-data v0.18.1
   Compiling rand v0.5.5
   Compiling num_cpus v1.8.0
[RUSTC-TIMING] chalk_engine test:false 10.360>                        ] 75/128
   Compiling rand v0.4.3
[RUSTC-TIMING] num_cpus test:false 2.494======>                       ] 76/128
   Compiling atty v0.2.11
[RUSTC-TIMING] atty test:false 0.503==========>                       ] 77/128
   Compiling jobserver v0.1.12
[RUSTC-TIMING] rls_data test:false 3.666=======>                      ] 78/128
   Compiling memmap v0.6.2
[RUSTC-TIMING] memmap test:false 1.366=========>                      ] 79/128
   Compiling rand_chacha v0.1.0
   Compiling rand_pcg v0.1.1
[RUSTC-TIMING] rand test:false 6.566============>                     ] 82/128
   Compiling rand v0.6.1
[RUSTC-TIMING] jobserver test:false 5.731========>                    ] 83/128
[RUSTC-TIMING] backtrace_sys test:false 0.286====>                    ] 84/128
   Compiling env_logger v0.5.13
[RUSTC-TIMING] serialize test:false 15.689========>                   ] 85/128
[RUSTC-TIMING] miniz_sys test:false 0.300=========>                   ] 86/128
   Compiling backtrace v0.3.11
   Compiling flate2 v1.0.6
[RUSTC-TIMING] rand test:false 10.056===============>                 ] 90/128
[RUSTC-TIMING] rand_chacha test:false 1.021=========>                 ] 91/128
[RUSTC-TIMING] rand_pcg test:false 0.506=============>                ] 92/128
   Compiling parking_lot_core v0.3.0
[RUSTC-TIMING] backtrace test:false 2.871=============>               ] 94/128
[RUSTC-TIMING] flate2 test:false 3.795================>               ] 95/128
[RUSTC-TIMING] env_logger test:false 5.287=============>              ] 96/128
[RUSTC-TIMING] parking_lot_core test:false 2.525=======>              ] 97/128
   Compiling parking_lot v0.6.4
[RUSTC-TIMING] rustc_rayon_core test:false 5.417=======>              ] 98/128
   Compiling rustc-rayon v0.1.1
[RUSTC-TIMING] parking_lot test:false 1.951=============>             ] 99/128
[RUSTC-TIMING] rand test:false 7.732===================>             ] 100/128
   Compiling tempfile v3.0.5
[RUSTC-TIMING] rustc_rayon test:false 6.435=============>            ] 101/128
   Compiling rustc_data_structures v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures)
[RUSTC-TIMING] tempfile test:false 2.131================>            ] 102/128
[RUSTC-TIMING] rustc_data_structures test:false 7.160====>           ] 103/128
   Compiling arena v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libarena)
[RUSTC-TIMING] arena test:false 0.876====================>           ] 104/128
   Compiling syntax_pos v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_pos)
[RUSTC-TIMING] syntax_pos test:false 7.404================>          ] 105/128
   Compiling rustc_errors v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_errors)
[RUSTC-TIMING] rustc_errors test:false 12.854=============>          ] 106/128
[RUSTC-TIMING] rustc_target test:false 28.047=============>          ] 107/128
[RUSTC-TIMING] syntax test:false 87.957====================>         ] 108/128
   Compiling syntax_ext v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_ext)
[RUSTC-TIMING] syntax_ext test:false 36.192================>         ] 109/128
[RUSTC-TIMING] rustc test:false 403.091=====================>        ] 110/128
   Compiling rustc_mir v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
   Compiling rustc_typeck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
   Compiling rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
[RUSTC-TIMING] rustc_lint test:false 88.288=================>        ] 111/128
   Compiling rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
[RUSTC-TIMING] rustc_incremental test:false 104.622==========>       ] 112/128
   Compiling rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
[RUSTC-TIMING] rustc_allocator test:false 35.524=============>       ] 113/128
[RUSTC-TIMING] rustc_traits test:false 150.111===============>       ] 114/128
[RUSTC-TIMING] rustc_metadata test:false 274.410==============>      ] 115/128
   Compiling rustc_resolve v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
[RUSTC-TIMING] rustc_typeck test:false 279.033================>      ] 116/128
   Compiling rustc_plugin v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
[RUSTC-TIMING] rustc_plugin test:false 26.788==================>     ] 117/128
   Compiling rustc_privacy v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
[RUSTC-TIMING] rustc_resolve test:false 81.722=================>     ] 118/128
[RUSTC-TIMING] rustc_privacy test:false 56.679==================>    ] 119/128
[RUSTC-TIMING] rustc_mir test:false 431.846=====================>    ] 120/128
   Compiling rustc_codegen_utils v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
   Compiling rustc_borrowck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_borrowck)
   Compiling rustc_passes v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
[RUSTC-TIMING] rustc_codegen_utils test:false 46.629============>    ] 121/128
   Compiling rustc_save_analysis v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
[RUSTC-TIMING] rustc_passes test:false 50.451====================>   ] 122/128
   Compiling rustc_codegen_ssa v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
[RUSTC-TIMING] rustc_borrowck test:false 63.321==================>   ] 123/128
[RUSTC-TIMING] rustc_save_analysis test:false 49.106==============>  ] 124/128
[RUSTC-TIMING] rustc_codegen_ssa test:false 78.626================>  ] 125/128
[RUSTC-TIMING] rustc_driver test:false 80.522======================> ] 126/128
   Compiling rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
[RUSTC-TIMING] rustc_binary test:false 1.148=======================> ] 127/128
    Finished release [optimized] target(s) in 20m 01s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 1202.025
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling libc v0.2.46
   Compiling build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
   Compiling cc v1.0.25
   Compiling rustc_codegen_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_llvm)
   Compiling rustc-demangle v0.1.10
[RUSTC-TIMING] rustc_demangle test:false 3.052                          ] 3/15
[RUSTC-TIMING] libc test:false 4.536>                                   ] 6/15
   Compiling num_cpus v1.8.0
   Compiling memmap v0.6.2
[RUSTC-TIMING] memmap test:false 1.154==>                               ] 7/15
   Compiling rustc_llvm v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_llvm)
[RUSTC-TIMING] num_cpus test:false 2.761=======>                        ] 9/15
[RUSTC-TIMING] cc test:false 17.623===================>                ] 11/15
[RUSTC-TIMING] rustc_llvm test:false 0.571====================>        ] 13/15
[RUSTC-TIMING] rustc_codegen_llvm test:false 88.282===============>    ] 14/15
    Finished release [optimized] target(s) in 2m 23s
[TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" } -- 143.438
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[TIMING] Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 0.526
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.25
   Compiling core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
   Compiling libc v0.2.46
   Compiling build_helper v0.1.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/build_helper)
   Compiling unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
   Compiling compiler_builtins v0.1.4
   Compiling cmake v0.1.33
   Compiling backtrace-sys v0.1.27
   Compiling std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
   Compiling rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
   Compiling rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
   Compiling rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
   Compiling rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
[RUSTC-TIMING] core test:false 170.140========>                        ] 21/36
   Compiling rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.216               ] 22/36
[RUSTC-TIMING] libc test:false 4.598=============>                     ] 23/36
[RUSTC-TIMING] compiler_builtins test:false 7.294==>                   ] 24/36
   Compiling alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
   Compiling rustc-demangle v0.1.10
   Compiling panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
[RUSTC-TIMING] backtrace_sys test:false 0.250=======>                  ] 25/36
[RUSTC-TIMING] panic_abort test:false 0.258===========>                ] 26/36
[RUSTC-TIMING] unwind test:false 0.535=================>               ] 27/36
[RUSTC-TIMING] rustc_demangle test:false 2.225===========>             ] 28/36
[RUSTC-TIMING] alloc test:false 26.448====================>            ] 29/36
   Compiling panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
[RUSTC-TIMING] rustc_tsan test:false 0.194==================>          ] 30/36
[RUSTC-TIMING] rustc_lsan test:false 0.205
[RUSTC-TIMING] rustc_asan test:false 0.192=====================>       ] 32/36
[RUSTC-TIMING] rustc_msan test:false 0.223=======================>     ] 33/36
[RUSTC-TIMING] panic_unwind test:false 1.027======================>    ] 34/36
[RUSTC-TIMING] std test:false 43.306================================>  ] 35/36
    Finished release [optimized] target(s) in 4m 10s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 250.091
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
   Compiling getopts v0.2.17
   Compiling term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
[RUSTC-TIMING] getopts test:false 6.395                                  ] 0/4
[RUSTC-TIMING] term test:false 8.528                                     ] 1/4
[RUSTC-TIMING] proc_macro test:false 29.807                              ] 2/4
   Compiling test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
[RUSTC-TIMING] test test:false 12.916====================>               ] 3/4
    Finished release [optimized] target(s) in 43.65s
Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } } -- 43.704
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cfg-if v0.1.6
   Compiling libc v0.2.46
   Compiling nodrop v0.1.12
   Compiling void v1.0.2
   Compiling memoffset v0.2.1
[RUSTC-TIMING] memoffset test:false 0.200                              ] 0/128
   Compiling rand_core v0.3.0
[RUSTC-TIMING] cfg_if test:false 0.253                                 ] 1/128
   Compiling scopeguard v0.3.3
[RUSTC-TIMING] void test:false 0.339                                   ] 2/128
   Compiling lazy_static v1.2.0
[RUSTC-TIMING] nodrop test:false 0.358                                 ] 3/128
   Compiling stable_deref_trait v1.1.0
[RUSTC-TIMING] scopeguard test:false 0.452                             ] 4/128
   Compiling rustc-rayon-core v0.1.1
[RUSTC-TIMING] stable_deref_trait test:false 0.341                     ] 5/128
   Compiling semver-parser v0.7.0
[RUSTC-TIMING] lazy_static test:false 0.531                            ] 6/128
   Compiling bitflags v1.0.4
   Compiling byteorder v1.2.7
[RUSTC-TIMING] bitflags test:false 0.407                               ] 8/128
   Compiling either v1.5.0
[RUSTC-TIMING] rand_core test:false 2.546                              ] 9/128
   Compiling unicode-width v0.1.5
   Compiling graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
[RUSTC-TIMING] either test:false 1.419                                ] 11/128
   Compiling cc v1.0.25
[RUSTC-TIMING] unicode_width test:false 0.750                         ] 12/128
   Compiling crc32fast v1.1.2
[RUSTC-TIMING] byteorder test:false 4.160                             ] 13/128
   Compiling scoped-tls v0.1.2
[RUSTC-TIMING] scoped_tls test:false 0.720                            ] 14/128
   Compiling rustc_target v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_target)
[RUSTC-TIMING] graphviz test:false 4.480                              ] 16/128
   Compiling syntax v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax)
   Compiling lazy_static v0.2.11
   Compiling termcolor v1.0.4
   Compiling datafrog v2.0.1
[RUSTC-TIMING] lazy_static test:false 0.478                           ] 19/128
   Compiling rustc-demangle v0.1.10
   Compiling rustc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
   Compiling remove_dir_all v0.5.1
[RUSTC-TIMING] remove_dir_all test:false 0.261                        ] 22/128
   Compiling rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
[RUSTC-TIMING] rustc_demangle test:false 2.930                        ] 23/128
   Compiling rustc_metadata v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
[RUSTC-TIMING] datafrog test:false 4.672                              ] 25/128
   Compiling rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
   Compiling fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
[RUSTC-TIMING] termcolor test:false 5.538                             ] 27/128
   Compiling rustc-serialize v0.3.24
[RUSTC-TIMING] rustc_fs_util test:false 1.955                         ] 28/128
   Compiling rustc_platform_intrinsics v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_platform_intrinsics)
   Compiling quick-error v1.2.2
[RUSTC-TIMING] fmt_macros test:false 4.978                            ] 29/128
   Compiling rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
[RUSTC-TIMING] quick_error test:false 0.446                           ] 30/128
   Compiling crossbeam-utils v0.2.2
[RUSTC-TIMING] crossbeam_utils test:false 2.482                       ] 32/128
   Compiling log v0.4.6
[RUSTC-TIMING] log test:false 3.237                                   ] 33/128
   Compiling unreachable v1.0.0
[RUSTC-TIMING] unreachable test:false 0.448                           ] 34/128
   Compiling arrayvec v0.4.7
   Compiling owning_ref v0.3.3
[RUSTC-TIMING] arrayvec test:false 4.908                              ] 36/128
   Compiling log_settings v0.1.2
[RUSTC-TIMING] log_settings test:false 1.079                          ] 37/128
   Compiling rand_core v0.2.2
[RUSTC-TIMING] owning_ref test:false 1.456                            ] 39/128
   Compiling rand_isaac v0.1.1
[RUSTC-TIMING] rand_core test:false 0.263                             ] 40/128
   Compiling rand_hc v0.1.0
[RUSTC-TIMING] rand_hc test:false 2.002                               ] 41/128
   Compiling rand_xorshift v0.1.0
[RUSTC-TIMING] rand_isaac test:false 3.263                            ] 42/128
   Compiling rustc-hash v1.0.1
[RUSTC-TIMING] rand_xorshift test:false 1.546                         ] 44/128
   Compiling semver v0.9.0
[RUSTC-TIMING] rustc_hash test:false 0.574                            ] 47/128
   Compiling chalk-macros v0.1.0
[RUSTC-TIMING] chalk_macros test:false 2.282                          ] 48/128
   Compiling humantime v1.2.0
[RUSTC-TIMING] humantime test:false 4.658                             ] 53/128
   Compiling rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
[RUSTC-TIMING] rustc_cratesio_shim test:false 1.051                   ] 55/128
   Compiling ena v0.11.0
   Compiling smallvec v0.6.7
[RUSTC-TIMING] cc test:false 29.418==>                                ] 57/128
   Compiling miniz-sys v0.1.11
[RUSTC-TIMING] ena test:false 3.145===>                               ] 58/128
   Compiling backtrace-sys v0.1.27
[RUSTC-TIMING] smallvec test:false 3.095                              ] 59/128
   Compiling crossbeam-epoch v0.3.1
error: linking with `cc` failed: exit code: 1                         ] 60/128
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.0.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.1.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.10.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.11.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.12.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.13.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.14.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.15.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.2.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.3.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.4.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.5.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.6.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.7.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.8.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.build_script_build.esk6ip8b-cgu.9.rcgu.o" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/miniz-sys-032e52749628e572/build_script_build-032e52749628e572.4rij9q2ih77gw4kr.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib" "-Wl,--start-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-0edb9a858e8b5b7a.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-698121058592466d.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-dd1b46d0cadcac0a.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-1a31b8a739b910e7.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-992bde8cba56d336.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-81cb34f01cf8f125.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-db28f576b7ec755e.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-30ec7361264d49c1.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-590157cbd6eb9894.rlib" "-Wl,--end-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-8b218909f1de9b54.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
  = note: /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.0.rcgu.o): in function `cc::Build::try_compile':
          cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x409f): undefined reference to `<std::ffi::os_str::OsStr as core::cmp::PartialEq>::eq'
          /usr/bin/ld: cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x4274): undefined reference to `<std::env::SplitPaths<'a> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x43db): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x46a0): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.0.rcgu.o): in function `cc::Build::get_base_compiler':
          cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build17get_base_compiler17h49f32894743338f7E+0x1aa1): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.10.rcgu.o): in function `<alloc::boxed::Box<T> as core::fmt::Debug>::fmt':
          cc.2wi1bjme-cgu.10:(.text._ZN63_$LT$alloc..boxed..Box$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h34876ebdca0b33e8E+0xf): undefined reference to `<(dyn core::any::Any + core::marker::Send + 'static) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.12.rcgu.o): in function `std::io::Write::write_all':
          cc.2wi1bjme-cgu.12:(.text._ZN3std2io5Write9write_all17h0a1effd41fa565faE+0x12d): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.12.rcgu.o): in function `std::io::Write::write_fmt':
          cc.2wi1bjme-cgu.12:(.text._ZN3std2io5Write9write_fmt17h7d9d6ad1013ff40fE+0x62): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.12.rcgu.o): in function `<std::io::buffered::BufWriter<W> as core::ops::drop::Drop>::drop':
          cc.2wi1bjme-cgu.12:(.text._ZN79_$LT$std..io..buffered..BufWriter$LT$W$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h92bcc267c55f4ad1E+0x142): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::copy_from_slice':
          cc.2wi1bjme-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17hc45b8ef1c7927d2fE+0xb8): undefined reference to `<core::fmt::Arguments<'_> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::contains':
          cc.2wi1bjme-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8contains17hc3d4f91abc33a096E+0x19): undefined reference to `<std::ffi::os_str::OsString as core::cmp::PartialEq>::eq'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: Could not compile `miniz-sys`.
warning: build failed, waiting for other jobs to finish...
error: linking with `cc` failed: exit code: 1                         ] 60/128
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.0.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.1.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.10.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.11.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.12.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.13.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.14.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.15.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.2.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.3.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.4.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.5.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.6.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.7.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.8.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.build_script_build.8ibqsp76-cgu.9.rcgu.o" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-5693778936355823/build_script_build-5693778936355823.7t662ucmt2vidbb.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib" "-Wl,--start-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-0edb9a858e8b5b7a.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-698121058592466d.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-dd1b46d0cadcac0a.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-1a31b8a739b910e7.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-992bde8cba56d336.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-81cb34f01cf8f125.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-db28f576b7ec755e.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-30ec7361264d49c1.rlib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-590157cbd6eb9894.rlib" "-Wl,--end-group" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-8b218909f1de9b54.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
  = note: /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.0.rcgu.o): in function `cc::Build::try_compile':
          cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x409f): undefined reference to `<std::ffi::os_str::OsStr as core::cmp::PartialEq>::eq'
          /usr/bin/ld: cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x4274): undefined reference to `<std::env::SplitPaths<'a> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x43db): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build11try_compile17h8d4f9894750a7b61E+0x46a0): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.0.rcgu.o): in function `cc::Build::get_base_compiler':
          cc.2wi1bjme-cgu.0:(.text._ZN2cc5Build17get_base_compiler17h49f32894743338f7E+0x1aa1): undefined reference to `<std::path::Display<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.10.rcgu.o): in function `<alloc::boxed::Box<T> as core::fmt::Debug>::fmt':
          cc.2wi1bjme-cgu.10:(.text._ZN63_$LT$alloc..boxed..Box$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h34876ebdca0b33e8E+0xf): undefined reference to `<(dyn core::any::Any + core::marker::Send + 'static) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.12.rcgu.o): in function `std::io::Write::write_all':
          cc.2wi1bjme-cgu.12:(.text._ZN3std2io5Write9write_all17h0a1effd41fa565faE+0x12d): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.12.rcgu.o): in function `std::io::Write::write_fmt':
          cc.2wi1bjme-cgu.12:(.text._ZN3std2io5Write9write_fmt17h7d9d6ad1013ff40fE+0x62): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.12.rcgu.o): in function `<std::io::buffered::BufWriter<W> as core::ops::drop::Drop>::drop':
          cc.2wi1bjme-cgu.12:(.text._ZN79_$LT$std..io..buffered..BufWriter$LT$W$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h92bcc267c55f4ad1E+0x142): undefined reference to `std::error::<impl core::convert::From<&'b str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Send + core::marker::Sync + 'a)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::copy_from_slice':
          cc.2wi1bjme-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17hc45b8ef1c7927d2fE+0xb8): undefined reference to `<core::fmt::Arguments<'_> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libcc-9519df3cf4247a6a.rlib(cc-9519df3cf4247a6a.cc.2wi1bjme-cgu.15.rcgu.o): in function `core::slice::<impl [T]>::contains':
          cc.2wi1bjme-cgu.15:(.text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8contains17hc3d4f91abc33a096E+0x19): undefined reference to `<std::ffi::os_str::OsString as core::cmp::PartialEq>::eq'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: Could not compile `backtrace-sys`.
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] crossbeam_epoch test:false 6.165                       ] 60/128
[RUSTC-TIMING] rustc_serialize test:false 42.128                      ] 61/128
[RUSTC-TIMING] rustc_platform_intrinsics test:false 49.831            ] 62/128
error: build failed
command did not execute successfully: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build --incremental -j 5
Build completed unsuccessfully in 0:30:57

real	30m57.860s
user	78m3.026s
sys	1m15.202s

