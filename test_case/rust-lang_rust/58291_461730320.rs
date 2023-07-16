
λ git clone git@github.com:rust-analyzer/rust-analyzer
Cloning into 'rust-analyzer'...
remote: Enumerating objects: 12, done.
remote: Counting objects: 100% (12/12), done.
remote: Compressing objects: 100% (12/12), done.
remote: Total 25560 (delta 1), reused 4 (delta 0), pack-reused 25548
Receiving objects: 100% (25560/25560), 4.59 MiB | 5.92 MiB/s, done.
Resolving deltas: 100% (16039/16039), done.

11:36:57|~/tmp
λ cd rust-analyzer/

11:37:21|~/tmp/rust-analyzer|master✓
λ git checkout incremental-bug 
Branch 'incremental-bug' set up to track remote branch 'incremental-bug' from 'origin'.
Switched to a new branch 'incremental-bug'

11:37:25|~/tmp/rust-analyzer|incremental-bug✓
λ cd crates/ra_ide_api

11:37:31|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug✓
λ cargo check
   Compiling semver-parser v0.7.0
   Compiling libc v0.2.48
   Compiling autocfg v0.1.2
   Compiling proc-macro2 v0.4.27
    Checking rand_core v0.4.0
   Compiling unicode-xid v0.1.0
    Checking void v1.0.2
    Checking stable_deref_trait v1.1.1
    Checking cfg-if v0.1.6
    Checking remove_dir_all v0.5.1
   Compiling serde v1.0.87
   Compiling byteorder v1.3.1
   Compiling num-traits v0.2.6
   Compiling ryu v0.2.7
    Checking quick-error v1.2.2
    Checking spin v0.4.10
    Checking fnv v1.0.6
    Checking scopeguard v0.3.3
    Checking bit-vec v0.5.0
    Checking ucd-util v0.1.3
   Compiling unicode-segmentation v1.2.1
   Compiling arrayvec v0.4.10
    Checking bitflags v1.0.4
    Checking itoa v0.4.3
    Checking colosseum v0.2.2
    Checking either v1.5.0
    Checking nodrop v0.1.13
    Checking memoffset v0.2.1
    Checking difference v2.0.0
    Checking drop_bomb v0.1.4
    Checking indexmap v1.0.2
   Compiling version_check v0.1.5
   Compiling rayon-core v1.4.1
    Checking ra_arena v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_arena)
    Checking relative-path v0.4.0
    Checking superslice v0.1.0
   Compiling rayon v1.0.3
    Checking join_to_string v0.1.3
    Checking unreachable v1.0.0
    Checking owning_ref v0.4.0
    Checking crossbeam-utils v0.2.2
    Checking log v0.4.6
    Checking rand_core v0.3.1
    Checking rand_jitter v0.1.2
   Compiling semver v0.9.0
   Compiling rand_chacha v0.1.1
   Compiling rand v0.6.5
    Checking lazy_static v1.2.0
    Checking regex-syntax v0.6.5
    Checking bit-set v0.5.0
    Checking itertools v0.8.0
   Compiling heck v0.3.1
    Checking smallvec v0.6.8
   Compiling unicase v2.2.0
    Checking lock_api v0.1.5
    Checking rand_xorshift v0.1.1
    Checking rand_hc v0.1.0
    Checking rand_isaac v0.1.1
    Checking ena v0.11.0
   Compiling rustc_version v0.2.3
    Checking rand_os v0.1.2
    Checking wait-timeout v0.1.5
    Checking num_cpus v1.9.0
    Checking memmap v0.6.2
    Checking rustc-hash v1.0.1
    Checking crossbeam-epoch v0.3.1
   Compiling rand_pcg v0.1.1
   Compiling parking_lot_core v0.4.0
   Compiling quote v0.6.11
    Checking fst v0.3.3
    Checking crossbeam-deque v0.2.0
   Compiling syn v0.15.26
    Checking tempfile v3.0.5
    Checking rusty-fork v0.2.1
    Checking parking_lot v0.7.1
    Checking proptest v0.9.0
   Compiling serde_derive v1.0.87
   Compiling derive-new v0.5.6
   Compiling salsa-macros v0.10.0
    Checking salsa v0.10.0
    Checking text_unit v0.1.6
    Checking smol_str v0.1.9
    Checking serde_json v1.0.38
    Checking ra_tt v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_tt)
    Checking rowan v0.3.3
    Checking ra_text_edit v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_text_edit)
    Checking ra_syntax v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_syntax)
    Checking test_utils v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/test_utils)
    Checking ra_db v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_db)
    Checking ra_mbe v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_mbe)
    Checking ra_ide_api_light v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api_light)
    Checking ra_hir v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_hir)
    Checking ra_assists v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_assists)
    Checking ra_ide_api v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api)
    Finished dev [unoptimized + debuginfo] target(s) in 32.79s

11:38:06|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug✓
λ env CARGO_INCREMENTAL=0 cargo check
    Checking ra_arena v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_arena)
    Checking ra_tt v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_tt)
    Checking test_utils v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/test_utils)
    Checking ra_text_edit v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_text_edit)
    Checking ra_syntax v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_syntax)
    Checking ra_db v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_db)
    Checking ra_mbe v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_mbe)
    Checking ra_ide_api_light v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api_light)
    Checking ra_hir v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_hir)
    Checking ra_assists v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_assists)
    Checking ra_ide_api v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api)
    Finished dev [unoptimized + debuginfo] target(s) in 6.31s

11:38:22|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug✓
λ nvim src/impls.rs # add a blank line before pub(crate) fn goto_implementation

11:44:30|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug⚡*
λ git diff
diff --git a/crates/ra_ide_api/src/impls.rs b/crates/ra_ide_api/src/impls.rs
index 4fb05413..a1780017 100644
--- a/crates/ra_ide_api/src/impls.rs
+++ b/crates/ra_ide_api/src/impls.rs
@@ -7,6 +7,7 @@ use hir::{db::HirDatabase, source_binder};
 
 use crate::{FilePosition, NavigationTarget, db::RootDatabase, RangeInfo};
 
+
 pub(crate) fn goto_implementation(
     db: &RootDatabase,
     position: FilePosition,

11:38:38|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug⚡*
λ cargo check
    Checking ra_arena v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_arena)
    Checking ra_tt v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_tt)
    Checking test_utils v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/test_utils)
    Checking ra_text_edit v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_text_edit)
    Checking ra_syntax v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_syntax)
    Checking ra_mbe v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_mbe)
    Checking ra_db v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_db)
    Checking ra_ide_api_light v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api_light)
    Checking ra_hir v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_hir)
    Checking ra_assists v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_assists)
    Checking ra_ide_api v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api)
error[E0599]: no method named `parse` found for type `&db::RootDatabase` in the current scope
  --> crates/ra_ide_api/src/impls.rs:15:19
   |
15 |     let file = db.parse(position.file_id);
   |                   ^^^^^
   |
   = note: the method `parse` exists but the following trait bounds were not satisfied:
           `db::RootDatabase : ra_db::SourceDatabase`
           `&db::RootDatabase : ra_db::SourceDatabase`
           `db::RootDatabase : ra_db::SourceDatabase`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `parse`, perhaps you need to implement it:
           candidate #1: `ra_db::SourceDatabase`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: Could not compile `ra_ide_api`.

To learn more, run the command again with --verbose.

11:44:31|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug⚡*
λ env CARGO_INCREMENTAL=0 cargo check
    Checking ra_arena v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_arena)
    Checking ra_tt v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_tt)
    Checking test_utils v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/test_utils)
    Checking ra_text_edit v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_text_edit)
    Checking ra_syntax v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_syntax)
    Checking ra_db v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_db)
    Checking ra_mbe v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_mbe)
    Checking ra_ide_api_light v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api_light)
    Checking ra_hir v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_hir)
    Checking ra_assists v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_assists)
    Checking ra_ide_api v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api)
    Finished dev [unoptimized + debuginfo] target(s) in 5.93s

11:45:16|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug⚡*
λ cargo check
    Checking ra_arena v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_arena)
    Checking ra_tt v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_tt)
    Checking test_utils v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/test_utils)
    Checking ra_text_edit v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_text_edit)
    Checking ra_syntax v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_syntax)
    Checking ra_db v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_db)
    Checking ra_mbe v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_mbe)
    Checking ra_ide_api_light v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api_light)
    Checking ra_hir v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_hir)
    Checking ra_assists v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_assists)
    Checking ra_ide_api v0.1.0 (/home/matklad/tmp/rust-analyzer/crates/ra_ide_api)
error[E0599]: no method named `parse` found for type `&db::RootDatabase` in the current scope
  --> crates/ra_ide_api/src/impls.rs:15:19
   |
15 |     let file = db.parse(position.file_id);
   |                   ^^^^^
   |
   = note: the method `parse` exists but the following trait bounds were not satisfied:
           `db::RootDatabase : ra_db::SourceDatabase`
           `&db::RootDatabase : ra_db::SourceDatabase`
           `db::RootDatabase : ra_db::SourceDatabase`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `parse`, perhaps you need to implement it:
           candidate #1: `ra_db::SourceDatabase`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: Could not compile `ra_ide_api`.

To learn more, run the command again with --verbose.

11:45:23|~/tmp/rust-analyzer/crates/ra_ide_api|incremental-bug⚡*
λ rustc --version
rustc 1.32.0 (9fda7c223 2019-01-16)
