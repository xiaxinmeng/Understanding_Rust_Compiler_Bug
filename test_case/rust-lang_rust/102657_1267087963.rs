plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........
failures:

---- [mir-opt] src/test/mir-opt/inline/issue-78442.rs stdout ----
29 -         _2 = <fn() {foo} as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
30 -                                          // mir::Constant
31 -                                          // + span: $DIR/issue-78442.rs:11:5: 11:15
- -                                          // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r fn() {foo}, ()) -> <fn() {foo} as FnOnce<()>>::Output {<fn() {foo} as Fn<()>>::call}, val: Value(<ZST>) }
+ -                                          // + literal: Const { ty: for<'a> extern "rust-call" fn(&'a fn() {foo}, ()) -> <fn() {foo} as FnOnce<()>>::Output {<fn() {foo} as Fn<()>>::call}, val: Value(<ZST>) }
33 +         _2 = move (*_3)() -> [return: bb7, unwind: bb4]; // scope 1 at $SRC_DIR/core/src/ops/function.rs:LL:COL
35   


thread '[mir-opt] src/test/mir-opt/inline/issue-78442.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_78442.bar.Inline.diff', src/tools/compiletest/src/runtest.rs:3515:25


failures:
    [mir-opt] src/test/mir-opt/inline/issue-78442.rs
