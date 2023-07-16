plain
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
Diff in /checkout/library/std/src/net/mod.rs at line 36:
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use self::udp::UdpSocket;
 
-mod socket_addr;
-mod ip_addr;
 mod display_buffer;
+mod ip_addr;
 mod parser;
+mod socket_addr;
 mod tcp;
 #[cfg(test)]
 mod test;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/benches/binary_heap.rs" "/checkout/library/std/src/net/mod.rs" "/checkout/library/std/src/process.rs" "/checkout/library/std/src/net/test.rs" "/checkout/library/std/src/f32.rs" "/checkout/library/std/src/env/tests.rs" "/checkout/library/std/src/panic/tests.rs" "/checkout/library/alloc/benches/slice.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
