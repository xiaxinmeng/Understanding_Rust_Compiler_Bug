rust
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
    Finished dev [unoptimized] target(s) in 0.64s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libcore)
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

[RUSTC-TIMING] core test:false 18.285
   Compiling rustc-std-workspace-core v1.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/tools/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.104               ] 22/36
   Compiling compiler_builtins v0.1.4
   Compiling libc v0.2.46
[RUSTC-TIMING] libc test:false 2.687=============>                     ] 23/36
[RUSTC-TIMING] compiler_builtins test:false 3.835==>                   ] 24/36
   Compiling alloc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/liballoc)
   Compiling unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libunwind)
   Compiling backtrace-sys v0.1.27
   Compiling rustc-demangle v0.1.10
   Compiling panic_abort v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_abort)
[RUSTC-TIMING] backtrace_sys test:false 0.139=======>                  ] 25/36
[RUSTC-TIMING] panic_abort test:false 0.162===========>                ] 26/36
[RUSTC-TIMING] unwind test:false 0.177=================>               ] 27/36
[RUSTC-TIMING] rustc_demangle test:false 1.356===========>             ] 28/36
[RUSTC-TIMING] alloc test:false 2.431=====================>            ] 29/36
   Compiling rustc_msan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_msan)
   Compiling rustc_asan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_asan)
   Compiling panic_unwind v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libpanic_unwind)
   Compiling rustc_tsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_tsan)
   Compiling rustc_lsan v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lsan)
[RUSTC-TIMING] rustc_tsan test:false 0.132==================>          ] 30/36
[RUSTC-TIMING] rustc_msan test:false 0.147====================>        ] 31/36
[RUSTC-TIMING] rustc_asan test:false 0.147
[RUSTC-TIMING] rustc_lsan test:false 0.139=======================>     ] 33/36
[RUSTC-TIMING] panic_unwind test:false 0.315======================>    ] 34/36
   Compiling std v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libstd)
[RUSTC-TIMING] std test:false 6.673=================================>  ] 35/36
    Finished release [optimized] target(s) in 32.44s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 32.492
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling proc_macro v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libproc_macro)
   Compiling term v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libterm)
   Compiling getopts v0.2.17
[RUSTC-TIMING] getopts test:false 3.648                                  ] 0/4
[RUSTC-TIMING] term test:false 4.328                                     ] 1/4
[RUSTC-TIMING] proc_macro test:false 12.435                              ] 2/4
   Compiling test v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libtest)
[RUSTC-TIMING] test test:false 6.657=====================>               ] 3/4
    Finished release [optimized] target(s) in 19.77s
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } } -- 19.829
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling nodrop v0.1.12
   Compiling cfg-if v0.1.6
   Compiling rand_core v0.3.0
   Compiling lazy_static v1.2.0
   Compiling void v1.0.2
[RUSTC-TIMING] cfg_if test:false 0.115                                 ] 1/128
   Compiling scopeguard v0.3.3
[RUSTC-TIMING] nodrop test:false 0.133                                 ] 2/128
   Compiling memoffset v0.2.1
[RUSTC-TIMING] void test:false 0.177                                   ] 3/128
   Compiling stable_deref_trait v1.1.0
[RUSTC-TIMING] lazy_static test:false 0.263                            ] 5/128
   Compiling byteorder v1.2.7
[RUSTC-TIMING] scopeguard test:false 0.190                             ] 7/128
   Compiling unicode-width v0.1.5
[RUSTC-TIMING] memoffset test:false 0.181                              ] 8/128
   Compiling either v1.5.0
[RUSTC-TIMING] stable_deref_trait test:false 0.208                     ] 9/128
   Compiling bitflags v1.0.4
[RUSTC-TIMING] bitflags test:false 0.181                              ] 10/128
   Compiling graphviz v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libgraphviz)
[RUSTC-TIMING] unicode_width test:false 0.307                         ] 11/128
   Compiling scoped-tls v0.1.2
[RUSTC-TIMING] rand_core test:false 0.925                             ] 15/128
   Compiling termcolor v1.0.4
[RUSTC-TIMING] either test:false 0.590                                ] 17/128
   Compiling lazy_static v0.2.11
[RUSTC-TIMING] scoped_tls test:false 0.403                            ] 18/128
   Compiling datafrog v2.0.1
[RUSTC-TIMING] lazy_static test:false 0.237                           ] 20/128
   Compiling rustc-demangle v0.1.10
[RUSTC-TIMING] byteorder test:false 1.827                             ] 21/128
   Compiling remove_dir_all v0.5.1
[RUSTC-TIMING] graphviz test:false 1.930                              ] 22/128
   Compiling rustc_fs_util v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_fs_util)
[RUSTC-TIMING] remove_dir_all test:false 0.341                        ] 25/128
   Compiling fmt_macros v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libfmt_macros)
[RUSTC-TIMING] rustc_demangle test:false 1.832                        ] 26/128
[RUSTC-TIMING] datafrog test:false 1.942                              ] 27/128
   Compiling rustc-serialize v0.3.24
   Compiling quick-error v1.2.2
[RUSTC-TIMING] rustc_fs_util test:false 0.799                         ] 28/128
   Compiling rustc_platform_intrinsics v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_platform_intrinsics)
[RUSTC-TIMING] quick_error test:false 0.268                           ] 29/128
   Compiling cc v1.0.25
[RUSTC-TIMING] termcolor test:false 2.978                             ] 31/128
   Compiling libc v0.2.46
   Compiling crossbeam-utils v0.2.2
[RUSTC-TIMING] crossbeam_utils test:false 0.760                       ] 33/128
   Compiling log v0.4.6
[RUSTC-TIMING] fmt_macros test:false 2.800                            ] 34/128
   Compiling arrayvec v0.4.7
[RUSTC-TIMING] log test:false 1.560                                   ] 35/128
   Compiling unreachable v1.0.0
[RUSTC-TIMING] unreachable test:false 0.240                           ] 36/128
   Compiling rustc-rayon-core v0.1.1
   Compiling log_settings v0.1.2
[RUSTC-TIMING] arrayvec test:false 2.096                              ] 38/128
   Compiling owning_ref v0.3.3
[RUSTC-TIMING] log_settings test:false 0.497                          ] 40/128
   Compiling rustc_target v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_target)
   Compiling crc32fast v1.1.2
   Compiling rand_core v0.2.2
[RUSTC-TIMING] rand_core test:false 0.223                             ] 45/128
   Compiling rand_isaac v0.1.1
[RUSTC-TIMING] owning_ref test:false 0.718                            ] 46/128
   Compiling rand_xorshift v0.1.0
[RUSTC-TIMING] rand_xorshift test:false 0.830                         ] 47/128
   Compiling rand_hc v0.1.0
[RUSTC-TIMING] rand_isaac test:false 1.313                            ] 48/128
   Compiling syntax v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax)
   Compiling rustc v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc)
   Compiling chalk-macros v0.1.0
[RUSTC-TIMING] rand_hc test:false 1.436                               ] 51/128
   Compiling rustc-hash v1.0.1
[RUSTC-TIMING] rustc_hash test:false 0.346                            ] 52/128
   Compiling rustc_metadata v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_metadata)
[RUSTC-TIMING] chalk_macros test:false 1.382                          ] 53/128
   Compiling rustc_incremental v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_incremental)
   Compiling humantime v1.2.0
   Compiling rustc_driver v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_driver)
[RUSTC-TIMING] humantime test:false 1.886                             ] 57/128
   Compiling rustc_cratesio_shim v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_cratesio_shim)
[RUSTC-TIMING] rustc_cratesio_shim test:false 0.744                   ] 58/128
   Compiling ena v0.11.0
[RUSTC-TIMING] libc test:false 3.220==>                               ] 59/128
   Compiling smallvec v0.6.7
[RUSTC-TIMING] ena test:false 1.238====>                              ] 60/128
   Compiling crossbeam-epoch v0.3.1
[RUSTC-TIMING] smallvec test:false 1.539                              ] 61/128
   Compiling miniz-sys v0.1.11
[RUSTC-TIMING] crossbeam_epoch test:false 3.205                       ] 63/128
   Compiling backtrace-sys v0.1.27
[RUSTC-TIMING] cc test:false 16.274======>                            ] 65/128
   Compiling lock_api v0.1.3
[RUSTC-TIMING] crc32fast test:false 1.236>                            ] 66/128
   Compiling polonius-engine v0.6.2
[RUSTC-TIMING] lock_api test:false 1.027==>                           ] 67/128
   Compiling chalk-engine v0.9.0
   Compiling num_cpus v1.8.0
[RUSTC-TIMING] polonius_engine test:false 1.822                       ] 69/128
   Compiling rand v0.4.3
[RUSTC-TIMING] rustc_serialize test:false 19.692                      ] 70/128
   Compiling rand v0.5.5
[RUSTC-TIMING] num_cpus test:false 2.605====>                         ] 71/128
   Compiling atty v0.2.11
[RUSTC-TIMING] atty test:false 0.470========>                         ] 72/128
   Compiling jobserver v0.1.12
[RUSTC-TIMING] rand test:false 6.015========>                         ] 73/128
   Compiling memmap v0.6.2
[RUSTC-TIMING] memmap test:false 0.992=======>                        ] 74/128
   Compiling serialize v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libserialize)
[RUSTC-TIMING] chalk_engine test:false 9.106=>                        ] 75/128
   Compiling rustc_apfloat v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_apfloat)
[RUSTC-TIMING] jobserver test:false 5.056=====>                       ] 76/128
   Compiling crossbeam-deque v0.2.0
[RUSTC-TIMING] crossbeam_deque test:false 0.746=>                     ] 80/128
[RUSTC-TIMING] miniz_sys test:false 0.351=======>                     ] 81/128
[RUSTC-TIMING] backtrace_sys test:false 0.265===>                     ] 82/128
   Compiling rls-span v0.4.0
[RUSTC-TIMING] rustc_platform_intrinsics test:false 28.629            ] 83/128
   Compiling env_logger v0.5.13
[RUSTC-TIMING] rls_span test:false 1.840=========>                    ] 84/128
   Compiling rand_chacha v0.1.0
[RUSTC-TIMING] rand test:false 10.769=============>                   ] 85/128
   Compiling rand_pcg v0.1.1
   Compiling rand v0.6.1
   Compiling flate2 v1.0.6
[RUSTC-TIMING] rustc_apfloat test:false 6.250======>                  ] 89/128
   Compiling backtrace v0.3.11
[RUSTC-TIMING] flate2 test:false 3.716==============>                 ] 90/128
[RUSTC-TIMING] env_logger test:false 5.725==========>                 ] 91/128
   Compiling rls-data v0.18.1
   Compiling parking_lot_core v0.3.0
[RUSTC-TIMING] rustc_rayon_core test:false 5.446=====>                ] 92/128
[RUSTC-TIMING] backtrace test:false 3.018============>                ] 93/128
[RUSTC-TIMING] rand_pcg test:false 0.580==============>               ] 94/128
[RUSTC-TIMING] rand_chacha test:false 0.982===========>               ] 95/128
[RUSTC-TIMING] parking_lot_core test:false 3.050=======>              ] 96/128
[RUSTC-TIMING] rls_data test:false 3.511===============>              ] 97/128
   Compiling rustc-rayon v0.1.1
   Compiling parking_lot v0.6.4
[RUSTC-TIMING] serialize test:false 12.826=============>              ] 98/128
[RUSTC-TIMING] parking_lot test:false 1.969=============>             ] 99/128
[RUSTC-TIMING] rustc_rayon test:false 6.226============>             ] 100/128
   Compiling rustc_data_structures v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_data_structures)
[RUSTC-TIMING] rand test:false 6.778====================>            ] 101/128
   Compiling tempfile v3.0.5
[RUSTC-TIMING] tempfile test:false 1.973================>            ] 102/128
[RUSTC-TIMING] rustc_data_structures test:false 6.170====>           ] 103/128
   Compiling arena v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libarena)
[RUSTC-TIMING] arena test:false 0.818====================>           ] 104/128
   Compiling syntax_pos v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_pos)
[RUSTC-TIMING] syntax_pos test:false 7.008================>          ] 105/128
   Compiling rustc_errors v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_errors)
[RUSTC-TIMING] rustc_errors test:false 12.638=============>          ] 106/128
[RUSTC-TIMING] rustc_target test:false 26.383=============>          ] 107/128
[RUSTC-TIMING] syntax test:false 87.246====================>         ] 108/128
   Compiling syntax_ext v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/libsyntax_ext)
[RUSTC-TIMING] syntax_ext test:false 36.262================>         ] 109/128
[RUSTC-TIMING] rustc test:false 403.460=====================>        ] 110/128
   Compiling rustc_mir v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_mir)
   Compiling rustc_typeck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_typeck)
   Compiling rustc_lint v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_lint)
[RUSTC-TIMING] rustc_lint test:false 87.772=================>        ] 111/128
   Compiling rustc_allocator v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_allocator)
[RUSTC-TIMING] rustc_incremental test:false 105.566==========>       ] 112/128
   Compiling rustc_traits v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_traits)
[RUSTC-TIMING] rustc_allocator test:false 35.018=============>       ] 113/128
[RUSTC-TIMING] rustc_traits test:false 146.747===============>       ] 114/128
[RUSTC-TIMING] rustc_metadata test:false 275.328==============>      ] 115/128
   Compiling rustc_plugin v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_plugin)
[RUSTC-TIMING] rustc_plugin test:false 25.848=================>      ] 116/128
   Compiling rustc_resolve v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_resolve)
[RUSTC-TIMING] rustc_mir test:false 403.656====================>     ] 117/128
   Compiling rustc_codegen_utils v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_utils)
   Compiling rustc_passes v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_passes)
   Compiling rustc_borrowck v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_borrowck)
[RUSTC-TIMING] rustc_resolve test:false 107.162================>     ] 118/128
[RUSTC-TIMING] rustc_typeck test:false 421.062==================>    ] 119/128
[RUSTC-TIMING] rustc_codegen_utils test:false 49.845============>    ] 120/128
[RUSTC-TIMING] rustc_passes test:false 51.817===================>    ] 121/128
   Compiling rustc_privacy v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_privacy)
   Compiling rustc_save_analysis v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/librustc_codegen_ssa)
[RUSTC-TIMING] rustc_borrowck test:false 64.116==================>   ] 122/128
[RUSTC-TIMING] rustc_privacy test:false 36.560===================>   ] 123/128
[RUSTC-TIMING] rustc_save_analysis test:false 51.470==============>  ] 124/128
[RUSTC-TIMING] rustc_codegen_ssa test:false 69.226================>  ] 125/128
[RUSTC-TIMING] rustc_driver test:false 63.908======================> ] 126/128
   Compiling rustc-main v0.0.0 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc)
error: linking with `cc` failed: exit code: 1======================> ] 127/128
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-86cfcdb180ca1244.4bhpffqk3funqhcw.rcgu.o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-86cfcdb180ca1244.4bnnlzdzsdhovhb8.rcgu.o" "-o" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-86cfcdb180ca1244" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-64431815b1652f33/out" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-5c1d73718543201d/out" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-d94f0ceba3b5a89b" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_traits-a8773236c481a551" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_save_analysis-6202b8eecfed4af5" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_codegen_utils-ec8e74a195ef7344" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_resolve-d37bfd4921b228a0" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_incremental-de7c86489cd1b79b" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_privacy-b85faedc4ff48bd9" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_typeck-c4ad385f41519a05" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_platform_intrinsics-e62e309067db2736" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_plugin-d84f7948491fa69e" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_metadata-7a8d618be3637136" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_ext-e59e3e461f4fb265" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_lint-bd52473efcc64668" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_passes-2ee491130475de2d" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_borrowck-03642d0ef468a1c2" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_mir-354544bfbcea27fc" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_allocator-a3c03bbba7fe7cc7" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc-282236f4f1f46ca9" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ltest-f3467d7206e00635" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lterm-191793eb2af4b9c0" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_fs_util-c9dfb87f256b1ada" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax-a744286dd2425515" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_errors-e7d774cc7807d9b2" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lsyntax_pos-936239c3e5bfd9ca" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_target-9a73c2e4a0117f6a" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lfmt_macros-724be23d12ec6248" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-larena-4ae1c3a2423060a6" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_data_structures-7c6c775edbed4c1f" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_cratesio_shim-6fe5b72986d6800e" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lgraphviz-a2d84108033452cc" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lserialize-44f98e624dadc81a" "-Wl,--start-group" "-L" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-6d610542ea1eb444" "-Wl,--end-group" "-Wl,-Bstatic" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-d261cb8259a57d68.rlib" "-Wl,-Bdynamic" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<(dyn core::any::Any + core::marker::Send + ReStatic) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<std::ffi::os_str::OsStr as core::cmp::PartialEq<std::ffi::os_str::OsStr>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-6fe5b72986d6800e.so: undefined reference to `<core::str::Bytes<ReEarlyBound(0, 'a)> as core::iter_private::TrustedRandomAccess>::may_have_side_effect'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-354544bfbcea27fc.so: undefined reference to `<core::fmt::Formatter<ReEarlyBound(0, '_)> as core::fmt::Write>::write_fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-354544bfbcea27fc.so: undefined reference to `<core::fmt::Formatter<ReEarlyBound(0, '_)> as core::fmt::Write>::write_char'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<core::time::Duration as core::ops::arith::Add<core::time::Duration>>::add'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e7d774cc7807d9b2.so: undefined reference to `<std::io::stdio::StdoutLock<ReEarlyBound(0, 'a)> as std::io::Write>::write'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<alloc::vec::Vec<u8> as core::convert::From<&ReEarlyBound(0, 'a) str>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<core::time::Duration as core::ops::arith::Sub<core::time::Duration>>::sub'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9a73c2e4a0117f6a.so: undefined reference to `<std::env::SplitPaths<ReEarlyBound(0, 'a)> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<core::fmt::Formatter<ReEarlyBound(0, '_)> as core::fmt::Write>::write_str'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-6fe5b72986d6800e.so: undefined reference to `<core::fmt::Arguments<ReEarlyBound(0, '_)> as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<std::path::Path as core::cmp::PartialEq<std::path::Path>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<std::path::PathBuf as core::cmp::PartialEq<std::path::PathBuf>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<&ReEarlyBound(0, 'a) std::fs::File as std::io::Write>::write'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-6fe5b72986d6800e.so: undefined reference to `<core::str::Bytes<ReEarlyBound(0, 'a)> as core::iter_private::TrustedRandomAccess>::get_unchecked'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-c4ad385f41519a05.so: undefined reference to `<&ReEarlyBound(1, 'b) alloc::string::String as core::str::pattern::Pattern<ReEarlyBound(0, 'a)>>::into_searcher'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<std::io::Guard<ReEarlyBound(0, 'a)> as core::ops::drop::Drop>::drop'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-936239c3e5bfd9ca.so: undefined reference to `<alloc::string::Drain<ReEarlyBound(0, 'a)> as core::ops::drop::Drop>::drop'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-de7c86489cd1b79b.so: undefined reference to `<std::sys::unix::time::Timespec as core::cmp::PartialEq<std::sys::unix::time::Timespec>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<&ReEarlyBound(0, 'a) std::fs::File as std::io::Read>::read'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-6d610542ea1eb444.so: undefined reference to `<rustc_demangle::Demangle<'a> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `std::error::<impl core::convert::From<&ReEarlyBound(1, 'b) str> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Sync + core::marker::Send + ReEarlyBound(0, 'a))>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<alloc::string::String as core::convert::From<alloc::borrow::Cow<ReEarlyBound(0, 'a), str>>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-de7c86489cd1b79b.so: undefined reference to `<std::sys::unix::time::Timespec as core::cmp::PartialOrd<std::sys::unix::time::Timespec>>::partial_cmp'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<std::io::stdio::StderrLock<ReEarlyBound(0, 'a)> as std::io::Write>::write'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `std::error::<impl core::convert::From<alloc::string::String> for alloc::boxed::Box<(dyn std::error::Error + ReStatic)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<core::num::bignum::Big32x40 as core::cmp::PartialOrd<core::num::bignum::Big32x40>>::partial_cmp'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<std::path::Components<ReEarlyBound(0, 'a)> as core::iter::iterator::Iterator>::next'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libtest-f3467d7206e00635.so: undefined reference to `<std::time::Instant as core::ops::arith::Sub<std::time::Instant>>::sub'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-6202b8eecfed4af5.so: undefined reference to `<core::num::bignum::Big32x40 as core::cmp::PartialEq<core::num::bignum::Big32x40>>::eq'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-ec8e74a195ef7344.so: undefined reference to `<(dyn core::any::Any + ReStatic) as core::fmt::Debug>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-282236f4f1f46ca9.so: undefined reference to `<std::path::PathBuf as core::cmp::PartialOrd<std::path::PathBuf>>::partial_cmp'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<core::fmt::Arguments<ReEarlyBound(0, '_)> as core::fmt::Display>::fmt'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `std::error::<impl core::convert::From<alloc::string::String> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Sync + core::marker::Send + ReStatic)>>::from'
          /usr/bin/ld: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-d94f0ceba3b5a89b.so: undefined reference to `<std::path::Display<ReEarlyBound(0, 'a)> as core::fmt::Display>::fmt'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

[RUSTC-TIMING] rustc_binary test:false 1.118
error: Could not compile `rustc-main`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "5" "--release" "--features" "" "--manifest-path" "/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/bootstrap/debug/bootstrap build --incremental -j 5
Build completed unsuccessfully in 0:20:20

real	20m20.782s
user	59m49.989s
sys	0m49.369s
