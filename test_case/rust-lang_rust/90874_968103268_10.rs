
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_from(&self, position: Idx) -> RangeToInclusive<Idx> {
-        ..=self.end-position
+        ..=self.end - position
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/ops/mod.rs" "/checkout/library/core/src/ops/unsize.rs" "/checkout/library/core/src/stream/stream/mod.rs" "/checkout/library/core/src/ops/range.rs" "/checkout/library/core/src/stream/mod.rs" "/checkout/library/core/src/ops/bit.rs" "/checkout/library/core/src/stream/from_iter.rs" "/checkout/library/core/src/any.rs"` failed.
Build completed unsuccessfully in 0:00:11
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
