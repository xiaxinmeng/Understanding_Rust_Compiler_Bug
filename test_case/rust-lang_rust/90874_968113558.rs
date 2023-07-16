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
Diff in /checkout/library/core/src/ops/range.rs at line 98:
     }
 }
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> Range<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> Range<Idx> {
     /// Returns a new `Range` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 117:
     }
 }
 
 
-impl<Idx: Sub<Idx, Output=Idx> + Copy> Range<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> Range<Idx> {
     /// Returns a new `Range` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 239:
     }
 }
 
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeFrom<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeFrom<Idx> {
     /// Returns a new `RangeFrom` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 258:
     }
 }
 
 
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeFrom<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeFrom<Idx> {
     /// Returns a new `RangeFrom` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 358:
     }
 }
 
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeTo<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeTo<Idx> {
     /// Returns a new `RangeTo` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 377:
     }
 }
 
 
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeTo<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeTo<Idx> {
     /// Returns a new `RangeTo` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 472:
     pub(crate) exhausted: bool,
 }
 
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeInclusive<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeInclusive<Idx> {
     /// Returns a new `RangeInclusive` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 491:
     }
 }
 
 
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeInclusive<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeInclusive<Idx> {
     /// Returns a new `RangeInclusive` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 752:
     }
 }
 
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeToInclusive<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeToInclusive<Idx> {
     /// Returns a new `RangeToInclusive` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 771:
     }
 }
 
 
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeToInclusive<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeToInclusive<Idx> {
     /// Returns a new `RangeToInclusive` analogous to this range, but moved from the
     /// given position.
     ///
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/any.rs" "/checkout/library/core/src/ptr/non_null.rs" "/checkout/library/core/src/stream/stream/mod.rs" "/checkout/library/core/src/ops/unsize.rs" "/checkout/library/core/src/ops/range.rs" "/checkout/library/core/src/ops/mod.rs" "/checkout/library/core/src/ops/bit.rs" "/checkout/library/core/src/slice/iter/macros.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
