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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/any.rs at line 798:
 where
     I: TypeTag<'a>,
 {
-    let mut req: ConcreteRequisition<'a, I> = RequisitionImpl {
-        tagged: TagValue(None),
-    };
+    let mut req: ConcreteRequisition<'a, I> = RequisitionImpl { tagged: TagValue(None) };
     provider.provide(Requisition(&mut req));
     req.tagged.0
Diff in /checkout/library/core/src/any.rs at line 945:
     )*};
 }
 
 
-tagged_methods!(
-    dyn Tagged<'a>,
-    dyn Tagged<'a> + Send
-);
+tagged_methods!(dyn Tagged<'a>, dyn Tagged<'a> + Send);
 
 ///////////////////////////////////////////////////////////////////////////////
 // Requisition and its methods
Diff in /checkout/library/core/src/any.rs at line 1017:
     )*};
 
 
-req_methods!(
-    SendRequisition
-);
-);
+req_methods!(Requisition, SendRequisition);
 
 /// A concrete request for a tagged value. Can be coerced to `Requisition` to be
 /// passed to provider methods.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/any.rs" "/checkout/library/core/src/default.rs" "/checkout/library/core/src/option.rs" "/checkout/library/core/src/pin.rs" "/checkout/library/core/src/future/into_future.rs" "/checkout/library/core/src/future/poll_fn.rs" "/checkout/library/core/src/future/join.rs" "/checkout/library/core/src/bool.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
