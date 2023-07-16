
  Compiling style_traits v0.0.1 (file:///C:/projects/rust/build/ct/servo/components/style_traits)
warning: the type of this value must be known in this context
    --> components\style\gecko\wrapper.rs:1777:30
     |
1777 |         (self.0 as *const _).hash(state);
     |                              ^^^^
     |
note: lint level defined here
    --> components\style\lib.rs:26:9
     |
26   | #![deny(warnings)]
     |         ^^^^^^^^
     = note: #[warn(tyvar_behind_raw_pointer)] implied by #[warn(warnings)]
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #46906 <https://github.com/rust-lang/rust/issues/46906>
warning: the type of this value must be known in this context
   --> components\style\gecko_bindings\sugar\ownership.rs:112:42
    |
112 |         debug_assert!(!(ptr as *const _).is_null());
    |                                          ^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #46906 <https://github.com/rust-lang/rust/issues/46906>
error: failed to remove C:\projects\rust\build\ct\servo\target\debug\deps\style-24ff635d097fd20b.style-gecko_bindings-structs-root-js.rcgu.o: The system cannot find the file specified. (os error 2)
error: failed to remove C:\projects\rust\build\ct\servo\target\debug\deps\style-24ff635d097fd20b.style-gecko_bindings-structs-root-js.volatile.rcgu.o: The system cannot find the file specified. (os error 2)
error: failed to remove C:\projects\rust\build\ct\servo\target\debug\deps\style-24ff635d097fd20b.style-gecko_bindings-structs-root-js.rcgu.bytecode.encoded: The system cannot find the file specified. (os error 2)
error: failed to remove C:\projects\rust\build\ct\servo\target\debug\deps\style-24ff635d097fd20b.style-gecko_bindings-structs-root-js.volatile.rcgu.bytecode.encoded: The system cannot find the file specified. (os error 2)
warning: Error finalizing incremental compilation session directory `\\?\C:\projects\rust\build\ct\servo\target\debug\incremental\style-3dw81kydn5x5v\s-ex2oqkzms9-1tey9ap-working`: The system cannot find the file specified. (os error 2)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.25.0-dev running on x86_64-pc-windows-msvc
thread 'rustc' panicked at 'librustc\session\mod.rs:645: Trying to invalidate IncrCompSession `InvalidBecauseOfErrors { session_directory: "\\\\?\\C:\\projects\\rust\\build\\ct\\servo\\target\\debug\\incremental\\style-3dw81kydn5x5v\\s-ex2oqkzms9-1tey9ap-working" }`', librustc\session\mod.rs:1130:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Could not compile `style`.
