plain
Successfully built 6c4607f12fa8
Successfully tagged rust-ci:latest
Built container sha256:6c4607f12fa84fde5e6923ac794ec0f4d60959a831c1b9ba9ba0fdbb252b7a69
Uploading finished image to https://ci-caches.rust-lang.org/docker/55db90084233c2c6d3f2ddcc4c9f8b7997aecabd635a628a2c66bcbdc007ca9a5c55a9b5bd41c20a325974553b8aaefb2756d07d1f18bc6caea3814fbff20c8c
upload failed: - to s3://rust-lang-ci-sccache2/docker/55db90084233c2c6d3f2ddcc4c9f8b7997aecabd635a628a2c66bcbdc007ca9a5c55a9b5bd41c20a325974553b8aaefb2756d07d1f18bc6caea3814fbff20c8c Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
 Documenting std v0.0.0 (/checkout/library/std)
error: unescaped backtick
   --> library/std/src/sys_common/wtf8.rs:597:12
    |
597 |     /// or `b'\xFF' otherwise.
    |
    |
    = note: `-D rustdoc::unescaped-backtick` implied by `-D warnings`
help: the closing backtick of an inline code may be missing
    |
597 |     /// or `b'\xFF'` otherwise.
    |                    +
help: if you meant to use a literal backtick, escape it
    |
597 |     /// or \`b'\xFF' otherwise.

error: could not document `std`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.68.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=93ef5ecfbccdc6b6 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libaddr2line-8a7a2a8af698b2a9.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/liballoc-4f83edf0d93f6be0.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-f28c165aeeb5e95a.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-4eda2368a8a2decc.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libcore-b98de7aec7c46ce5.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libhashbrown-32fbed4d6c19bc4a.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/liblibc-67871cedd8ff5ccb.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-7d04af51791a6cd5.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libobject-b80afb0b27a1c1ef.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-4165f62430fdf2d0.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-43c9d56813117e2b.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-d1703573aa961b40.rmeta --extern std_detect=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libstd_detect-2662b14564e70170.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps/libunwind-9317f74ab3f170f2.rmeta -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' --document-private-items --document-hidden-items -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (b9e155996
  2022-12-18)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --cfg backtrace_in_libstd` (exit status: 1)
