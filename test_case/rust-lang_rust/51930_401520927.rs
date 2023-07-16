
---- doc::doc_target stdout ----
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe doc --target arm-unknown-linux-gnueabihf --verbose`
thread 'doc::doc_target' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout
--- stderr
 Documenting foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t484/foo)
     Running `rustdoc --crate-name foo src\lib.rs --target arm-unknown-linux-gnueabihf -o C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t484\foo\target\arm-unknown-linux-gnueabihf\doc -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t484\foo\target\arm-unknown-linux-gnueabihf\debug\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t484\foo\target\debug\deps`
error: requires `sized` lang_item
error: Could not document `foo`.
Caused by:
  process didn't exit successfully: `rustdoc --crate-name foo src\lib.rs --target arm-unknown-linux-gnueabihf -o C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t484\foo\target\arm-unknown-linux-gnueabihf\doc -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t484\foo\target\arm-unknown-linux-gnueabihf\debug\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t484\foo\target\debug\deps` (exit code: 101)
', tools\cargo\tests\testsuite\hamcrest.rs:13:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    doc::doc_target
