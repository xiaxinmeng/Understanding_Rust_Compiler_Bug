
│ │ │ │ │ │ │ │ │ │ │ │             BasicBlockData {
│ │ │ │ │ │ │ │ │ │ │ │                 statements: [
│ │ │ │ │ │ │ │ │ │ │ │                     StorageLive(_7),
│ │ │ │ │ │ │ │ │ │ │ │                     StorageLive(_8),
│ │ │ │ │ │ │ │ │ │ │ │                     _8 = &mut ((*_5).0: <T as A>::MyType),
│ │ │ │ │ │ │ │ │ │ │ │                     _7 = &raw mut (*_8),
│ │ │ │ │ │ │ │ │ │ │ │                 ],
│ │ │ │ │ │ │ │ │ │ │ │                 terminator: Some(
│ │ │ │ │ │ │ │ │ │ │ │                     Terminator {
│ │ │ │ │ │ │ │ │ │ │ │                         source_info: SourceInfo {
│ │ │ │ │ │ │ │ │ │ │ │                             span: /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53 (#30),
│ │ │ │ │ │ │ │ │ │ │ │                             scope: scope[6],
│ │ │ │ │ │ │ │ │ │ │ │                         },
│ │ │ │ │ │ │ │ │ │ │ │                         kind: _0 = std::ptr::drop_in_place::<<T as A>::MyType>(move _7) -> [return: bb8, unwind: bb4],
│ │ │ │ │ │ │ │ │ │ │ │                     },
│ │ │ │ │ │ │ │ │ │ │ │                 ),
│ │ │ │ │ │ │ │ │ │ │ │                 is_cleanup: false,
│ │ │ │ │ │ │ │ │ │ │ │             },
│ │ │ │ │ │ │ │ │ │ │ │             BasicBlockData {
│ │ │ │ │ │ │ │ │ │ │ │                 statements: [
│ │ │ │ │ │ │ │ │ │ │ │                     StorageDead(_7),
│ │ │ │ │ │ │ │ │ │ │ │                     StorageDead(_8),
│ │ │ │ │ │ │ │ │ │ │ │                 ],
│ │ │ │ │ │ │ │ │ │ │ │                 terminator: Some(
│ │ │ │ │ │ │ │ │ │ │ │                     Terminator {
│ │ │ │ │ │ │ │ │ │ │ │                         source_info: SourceInfo {
│ │ │ │ │ │ │ │ │ │ │ │                             span: /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:145:6: 145:6 (#0),
│ │ │ │ │ │ │ │ │ │ │ │                             scope: scope[0],
│ │ │ │ │ │ │ │ │ │ │ │                         },
│ │ │ │ │ │ │ │ │ │ │ │                         kind: goto -> bb2,
│ │ │ │ │ │ │ │ │ │ │ │                     },
│ │ │ │ │ │ │ │ │ │ │ │                 ),
│ │ │ │ │ │ │ │ │ │ │ │                 is_cleanup: false,
│ │ │ │ │ │ │ │ │ │ │ │             },
