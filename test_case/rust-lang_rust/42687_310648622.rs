
Environment: MSYS_BITS=64, SCRIPT=python x.py dist, RUST_CONFIGURE_ARGS=--build=x86_64-pc-windows-gnu --enable-extended, MINGW_URL=https://s3.amazonaws.com/rust-lang-ci/rust-ci-mirror, MINGW_ARCHIVE=x86_64-6.3.0-release-posix-seh-rt_v5-rev2.7z, MINGW_DIR=mingw64, DEPLOY=1

[00:56:10] Building stage2 tool cargo (x86_64-pc-windows-gnu)
[00:56:12]    Compiling pkg-config v0.3.9
[00:56:12]    Compiling hex v0.2.0
[00:56:13] error: Could not compile `hex`.
[00:56:13] Build failed, waiting for other jobs to finish...
[00:56:13] error: Could not compile `pkg-config`.
[00:56:13]
[00:56:13] To learn more, run the command again with --verbose.
[00:56:13]
[00:56:13]
[00:56:13] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage0/bin\\cargo.exe" "build" "-j" "2" "--target" "x86_64-pc-windows-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools\\cargo\\Cargo.toml"
[00:56:13] expected success, got: exit code: 101
[00:56:13]
