
[21418/207373](69) RUST x64-coverage-rust/no_rust_tests x64-coverage-rust/exe.unstripped/no_rust_tests x64-coverage-rust/no_rust_tests.build-id.stamp
FAILED: x64-coverage-rust/no_rust_tests x64-coverage-rust/exe.unstripped/no_rust_tests x64-coverage-rust/no_rust_tests.build-id.stamp
mkdir -p x64-coverage-rust/exe.unstripped && ../../build/tracer/output_cacher_local_wrapper.sh ../../prebuilt/third_party/python3/linux-x64/bin/python3.8 -S ../../build/tracer/restat_cacher.py --outputs x64-coverage-rust/no_rust_tests x64-coverage-rust/exe.unstripped/no_rust_tests x64-coverage-rust/no_rust_tests.build-id.stamp x64-coverage-rust/no_rust_tests.d -- ../../prebuilt/third_party/pyth...
error: linking with `../../prebuilt/third_party/clang/linux-x64/bin/lld` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="../../prebuilt/third_party/rust/linux-x64/lib/rustlib/x86_64-unknown-linux-gnu/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" VSLANG="1033" "../../prebuilt/third_party/clang/linux-x64/bin/lld" "-flavor" "gnu" "--build-id" "--hash-style=gnu" "-z" "max-page-size=4096" "-z" "now" "-z" "rodynamic" "-z" "separate-loadable-segments" "--pack-dyn-relocs=rel...

  = note: lld: error: undefined hidden symbol: __llvm_profile_counter_bias
          >>> referenced by InstrProfilingPlatformFuchsia.c:183 (/b/s/w/ir/x/w/fuchsia-third_party-rust/library/profiler_builtins/../../src/llvm-project/compiler-rt/lib/profile/InstrProfilingPlatformFuchsia.c:183)
