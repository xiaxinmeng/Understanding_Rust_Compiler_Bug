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
Diff in /checkout/library/alloc/src/rc.rs at line 1238:
     pub fn unwrap_or_clone(this: Self) -> T {
         Rc::try_unwrap(this).unwrap_or_else(|rc| (*rc).clone())
-
 }
 
 
 impl Rc<dyn Any> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/alloc/tests.rs" "/checkout/library/alloc/src/rc.rs" "/checkout/library/alloc/src/collections/vec_deque/ring_slices.rs" "/checkout/library/alloc/src/vec/splice.rs" "/checkout/library/alloc/src/collections/vec_deque/drain.rs" "/checkout/library/alloc/src/vec/spec_from_iter.rs" "/checkout/library/alloc/src/collections/vec_deque/iter_mut.rs" "/checkout/library/alloc/src/collections/linked_list.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
