
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: (MoveData { move_paths: [MovePath { place: _0 }, MovePath { place: _1 }, MovePath { place: _2 }, MovePath { place: _3 }, MovePath { place: _4 }, MovePath { place: _5 }, MovePath { place: _6 }, MovePath { place: _7 }], moves: [mp5@bb0[8], mp6@bb0[8], mp6@bb2[0], mp5@bb2[1], mp7@bb2[2], mp3@bb2[4], mp3@bb3[0], mp3@bb4[0], mp0@bb4[1]], loc_map: LocationMap { map: [[[], [], [], [], [], [], [], [], [mo0, mo1]], [[]], [[mo2], [mo3], [mo4], [], [mo5]], [[mo6]], [[mo7], [mo8]]] }, path_map: [[mo8], [], [], [mo5, mo6, mo7], [], [mo0, mo3], [mo1, mo2], [mo4]], rev_lookup: MovePathLookup { locals: [mp0, mp1, mp2, mp3, mp4, mp5, mp6, mp7], projections: {} }, inits: [mp1@src/main.rs:4:56: 6:6 (Deep), mp2@src/main.rs:4:57: 4:67 (Deep), mp3@src/main.rs:4:61: 4:66 (Deep), mp5@src/main.rs:5:9: 5:12 (Deep), mp7@src/main.rs:5:20: 5:26 (Deep), mp6@src/main.rs:5:20: 5:26 (Deep), mp4@src/main.rs:5:9: 5:27 (NonPanicPathOnly), mp0@src/main.rs:4:69: 6:6 (Deep)], init_loc_map: LocationMap { map: [[[], [in2], [], [in3], [], [], [in4], [in5], [in6]], [[]], [[], [], [], [in7], []], [[]], [[], []]] }, init_path_map: [[in7], [in0], [in1], [in2], [in6], [in3], [in5], [in4]] }, [IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/main.rs:4:61: 4:66, kind: BorrowedContent } }])', libcore/result.rs:945:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.0 (3eda71b00 2018-06-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
