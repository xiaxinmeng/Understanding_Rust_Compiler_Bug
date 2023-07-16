plain
   Doc-tests core
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2366:34
     |
2366 |             #[cfg_attr(all(test, target_endiant = "little"), assert_instr($little))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2378 |     impl_vec_unpack! { vec_vupkhsb (vector_signed_char) -> vector_signed_short [vupklsb, vupkhsb] }
     |
     |
     = note: `-D unexpected-cfgs` implied by `-D warnings`
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2367:34
     |
     |
2367 |             #[cfg_attr(all(test, target_endiant = "big"), assert_instr($big))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2378 |     impl_vec_unpack! { vec_vupkhsb (vector_signed_char) -> vector_signed_short [vupklsb, vupkhsb] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2366:34
     |
     |
2366 |             #[cfg_attr(all(test, target_endiant = "little"), assert_instr($little))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2379 |     impl_vec_unpack! { vec_vupklsb (vector_signed_char) -> vector_signed_short [vupkhsb, vupklsb] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2367:34
     |
     |
2367 |             #[cfg_attr(all(test, target_endiant = "big"), assert_instr($big))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2379 |     impl_vec_unpack! { vec_vupklsb (vector_signed_char) -> vector_signed_short [vupkhsb, vupklsb] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2366:34
     |
     |
2366 |             #[cfg_attr(all(test, target_endiant = "little"), assert_instr($little))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2380 |     impl_vec_unpack! { vec_vupkhsh (vector_signed_short) -> vector_signed_int [vupklsh, vupkhsh] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2367:34
     |
     |
2367 |             #[cfg_attr(all(test, target_endiant = "big"), assert_instr($big))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2380 |     impl_vec_unpack! { vec_vupkhsh (vector_signed_short) -> vector_signed_int [vupklsh, vupkhsh] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2366:34
     |
     |
2366 |             #[cfg_attr(all(test, target_endiant = "little"), assert_instr($little))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2381 |     impl_vec_unpack! { vec_vupklsh (vector_signed_short) -> vector_signed_int [vupkhsh, vupklsh] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected `cfg` condition name
    --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:2367:34
     |
     |
2367 |             #[cfg_attr(all(test, target_endiant = "big"), assert_instr($big))]
     |                                  |
     |                                  help: did you mean: `target_endian`
...
...
2381 |     impl_vec_unpack! { vec_vupklsh (vector_signed_short) -> vector_signed_int [vupkhsh, vupklsh] }
     |
     |
     = note: this error originates in the macro `impl_vec_unpack` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors


error: doctest failed, to rerun pass `-p core --doc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core --test /checkout/library/core/src/lib.rs --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ca9c16543ffcbc1c/out --test-args --quiet --test-args -Z --test-args unstable-options --test-args --format --test-args json --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-0573e3b43a3a5320.rlib --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librand-0f90125d4e867403.rlib --extern rand_xorshift=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librand_xorshift-6c792bcc3885f0c5.rlib -C embed-bitcode=no -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.71.0-nightly
  (fbbad75ad
  2023-05-25)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' '-Zcrate-attr=warn(rust_2018_idioms)' --error-format human` (exit status: 1)
