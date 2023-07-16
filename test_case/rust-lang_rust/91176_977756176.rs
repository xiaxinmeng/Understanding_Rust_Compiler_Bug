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
   |
51 |             counter++;
   |                     ^ expected expression

Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/hermit/stdio.rs" "/checkout/library/std/src/sys/hermit/args.rs" "/checkout/library/std/src/sys/hermit/mutex.rs" "/checkout/library/std/src/sys/hermit/net.rs" "/checkout/library/std/src/sys/hermit/alloc.rs" "/checkout/library/std/src/sys/hermit/mod.rs" "/checkout/library/std/src/sys/hermit/fs.rs" "/checkout/library/std/src/sys/sgx/abi/usercalls/raw.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
