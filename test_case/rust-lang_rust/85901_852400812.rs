plain
---- src/io/buffered/bufwriter.rs - io::buffered::bufwriter::BufWriter<W>::into_parts (line 326) stdout ----
error[E0658]: use of unstable library feature 'bufwriter_into_raw_parts'
  --> src/io/buffered/bufwriter.rs:334:48
   |
11 | let (recovered_writer, buffered_data) = stream.into_parts();
   |
   = note: see issue #80690 <https://github.com/rust-lang/rust/issues/80690> for more information
   = help: add `#![feature(bufwriter_into_raw_parts)]` to the crate attributes to enable

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:33
