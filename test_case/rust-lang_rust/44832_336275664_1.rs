
DEBUG:rustc_mir::borrow_check: MirBorrowckCtxt::process_terminator(bb3[0], \
    Terminator { source_info: SourceInfo { span: ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2, scope: scope0 }, \
                                           kind: drop(_2) -> bb2 }): \
    borrows in effect: [&'13_0rs (*_2)] borrows generated: [] inits: [_0, _3, _4, _6] uninits: [_0, _1, _2, _4, _5, _6]
error[E0505]: cannot move out of `x` because it is borrowed (Mir)
  --> ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2
   |
13 |     let y = &*x;
   |             --- borrow of `(*x)` occurs here
...
16 | }
   |  ^ move out of `x` occurs here
