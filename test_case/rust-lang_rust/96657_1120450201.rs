
     Running `/data/semarie/build-rust/build_dir/build/bootstrap/debug/rustc --crate-name std --edition=2021 library/std/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type dylib
--crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"'
--cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' -C metadata=378c766641f06cb7 -C extra-filename=-378c766641f06cb7
--out-dir /data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps --target x86_64-unknown-openbsd -L
dependency=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps -L dependency=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/release/deps --extern
addr2line=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libaddr2line-54699a61a4996e32.rlib --extern
alloc=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/liballoc-c1464684857de2fb.rlib --extern
cfg_if=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libcfg_if-911ca56735f69dbf.rlib --extern
compiler_builtins=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libcompiler_builtins-5e745fef8d64cade.rlib --extern
core=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libcore-c340d24d8e18ebf7.rlib --extern
hashbrown=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libhashbrown-24ee209395c036b6.rlib --extern
libc=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/liblibc-48a88cf05aef92e5.rlib --extern
miniz_oxide=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libminiz_oxide-eeb523364a36f721.rlib --extern
object=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libobject-6fcd64c1f2a7a79d.rlib --extern
panic_abort=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libpanic_abort-9c65896c13c96c61.rlib --extern
panic_unwind=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libpanic_unwind-1fc476adbfc951f7.rlib --extern
rustc_demangle=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/librustc_demangle-1fa31f1cfd9e0d48.rlib --extern
std_detect=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libstd_detect-e6ac865e361f7f7b.rlib --extern
unwind=/data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std/x86_64-unknown-openbsd/release/deps/libunwind-6af071432b46fe34.rlib --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zmacro-backtrace
-Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo --cfg
backtrace_in_libstd -L native=/usr/lib`
error: associated function is never used: `to_timespec`
   --> library/std/src/sys/unix/time.rs:129:12
    |
129 |     pub fn to_timespec(&self) -> Option<libc::timespec> {
    |            ^^^^^^^^^^^
    |
    = note: `-D dead-code` implied by `-D warnings`


Did not run successfully: exit status: 1
