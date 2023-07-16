plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/panic_unwind/src/emcc.rs at line 105:
 }
 
-
-
 // This is required by the compiler to exist (e.g., it's a lang item), but it's
 // never actually called by the compiler.  Emscripten EH doesn't use a
 // personality function at all, it instead uses __cxa_find_matching_catch.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/test/src/helpers/exit_code.rs" "/checkout/library/test/src/term/terminfo/mod.rs" "/checkout/library/test/src/term/terminfo/parm/tests.rs" "/checkout/library/test/src/tests.rs" "/checkout/library/test/src/stats/tests.rs" "/checkout/library/panic_unwind/src/emcc.rs" "/checkout/library/test/src/time.rs" "/checkout/library/test/src/term/terminfo/parser/compiled/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
