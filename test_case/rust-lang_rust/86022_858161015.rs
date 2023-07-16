
info: check src/librustdoc --stage 0 --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
    Checking rustdoc v0.0.0 (/home/joshua/rustc/src/librustdoc)
error: expected item, found `/`
 --> src/librustdoc/lib.rs:1:1
  |
1 | /
  | ^ expected item

error: aborting due to previous error

error: could not compile `rustdoc`

To learn more, run the command again with --verbose.
note: failed while building bootstrap::check::Rustdoc
failed to run: /home/joshua/rustc/build/bootstrap/debug/bootstrap check src/librustdoc
Build completed unsuccessfully in 0:00:01
