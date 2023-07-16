
[01:26:39] error[E0433]: failed to resolve: could not find `item_path` in `ty`
[01:26:39]    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:104:10
[01:26:39]     |
[01:26:39] 104 | impl ty::item_path::ItemPathBuffer for AbsolutePathBuffer {
[01:26:39]     |          ^^^^^^^^^ could not find `item_path` in `ty`
[01:26:39] 
[01:26:39] error[E0433]: failed to resolve: could not find `item_path` in `ty`
[01:26:39]    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:105:33
[01:26:39]     |
[01:26:39] 105 |     fn root_mode(&self) -> &ty::item_path::RootMode {
[01:26:39]     |                                 ^^^^^^^^^ could not find `item_path` in `ty`
[01:26:39] 
[01:26:39] error[E0433]: failed to resolve: could not find `item_path` in `ty`
[01:26:39]    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:106:30
[01:26:39]     |
[01:26:39] 106 |         const ABSOLUTE: &ty::item_path::RootMode = &ty::item_path::RootMode::Absolute;
[01:26:39]     |                              ^^^^^^^^^ could not find `item_path` in `ty`
[01:26:39] 
[01:26:39] error[E0433]: failed to resolve: could not find `item_path` in `ty`
[01:26:39]    --> src/tools/clippy/clippy_lints/src/utils/mod.rs:106:57
[01:26:39]     |
[01:26:39] 106 |         const ABSOLUTE: &ty::item_path::RootMode = &ty::item_path::RootMode::Absolute;
[01:26:39]     |                                                         ^^^^^^^^^ could not find `item_path` in `ty`
[01:26:39] 
[01:26:40] error: aborting due to 4 previous errors
