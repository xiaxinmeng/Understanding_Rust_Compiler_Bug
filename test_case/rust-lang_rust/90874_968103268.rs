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
Diff in /checkout/library/core/src/ops/range.rs at line 113:
     /// 