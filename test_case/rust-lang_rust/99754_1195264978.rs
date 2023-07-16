plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: variable `snapshot_before_last_ty` is assigned to, but never used
    |
    |
553 |         let mut snapshot_before_last_ty = self.create_snapshot_for_diagnostic();
    |
    |
    = note: `-D unused-variables` implied by `-D warnings`
    = note: consider using `_snapshot_before_last_ty` instead

error: value assigned to `snapshot_before_last_ty` is never read
    |
    |
592 |             snapshot_before_last_ty = self.create_snapshot_for_diagnostic();
    |
    |
    = note: `-D unused-assignments` implied by `-D warnings`
    = help: maybe it is overwritten before being read?
error: could not compile `rustc_parse` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_parse` due to 2 previous errors
Build completed unsuccessfully in 0:01:53
