
thread 'rustc' panicked at 'Found unstable fingerprints for item_children(veloren_common_base[97e7]): [Export { ident: userdata_dir#0, res: Def(Mod, DefId(96:3 ~ veloren_common_base[97e7]::userdata_dir)), span: /veloren/common/base/src/lib.rs:3:1: 3:22 (#0), vis: Public }, Export { ident: DummySpan#0, res: Def(Struct, DefId(96:18 ~ veloren_common_base[97e7]::DummySpan)), span: /veloren/common/base/src/lib.rs:61:1: 61:22 (#0), vis: Public }, Export { ident: DummySpan#0, res: Def(Ctor(Struct, Const), DefId(96:19 ~ veloren_common_base[97e7]::DummySpan::{constructor#0})), span: /veloren/common/base/src/lib.rs:61:1: 61:22 (#0), vis: Public }, Export { ident: GuardlessSpan#0, res: Def(Struct, DefId(96:21 ~ veloren_common_base[97e7]::GuardlessSpan)), span: /veloren/common/base/src/lib.rs:83:1: 86:2 (#0), vis: Public }, Export { ident: userdata_dir_workspace#0, res: Def(Macro(Bang), DefId(96:11 ~ veloren_common_base[97e7]::userdata_dir::userdata_dir_workspace)), span: /veloren/common/base/src/userdata_dir.rs:67:1: 75:2 (#0), vis: Public }, Export { ident: userdata_dir_no_workspace#0, res: Def(Macro(Bang), DefId(96:12 ~ veloren_common_base[97e7]::userdata_dir::userdata_dir_no_workspace)), span: /veloren/common/base/src/userdata_dir.rs:78:1: 86:2 (#0), vis: Public }, Export { ident: userdata_dir#0, res: Def(Fn, DefId(96:8 ~ veloren_common_base[97e7]::userdata_dir::userdata_dir)), span: /veloren/common/base/src/lib.rs:5:9: 5:35 (#0), vis: Public }, Export { ident: plot#0, res: Def(Macro(Bang), DefId(96:16 ~ veloren_common_base[97e7]::plot)), span: /veloren/common/base/src/lib.rs:10:1: 24:2 (#0), vis: Public }, Export { ident: span#0, res: Def(Macro(Bang), DefId(96:17 ~ veloren_common_base[97e7]::span)), span: /veloren/common/base/src/lib.rs:28:1: 59:2 (#0), vis: Public }, Export { ident: prof_span#0, res: Def(Macro(Bang), DefId(96:20 ~ veloren_common_base[97e7]::prof_span)), span: /veloren/common/base/src/lib.rs:66:1: 80:2 (#0), vis: Public }, Export { ident: no_guard_span#0, res: Def(Macro(Bang), DefId(96:29 ~ veloren_common_base[97e7]::no_guard_span)), span: /veloren/common/base/src/lib.rs:107:1: 123:2 (#0), vis: Public }]', /rustc/3e99439f4dacc8ba0d2ca48d221694362d587927/compiler/rustc_query_system/src/query/plumbing.rs:619:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.54 (3e99439 2021-05-17)

query stack during panic:
#0 [item_children] collecting child items of `veloren_common_base`
#1 [visible_parent_map] calculating the visible parent map
end of query stack
