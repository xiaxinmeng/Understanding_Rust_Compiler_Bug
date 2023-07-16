
% ( OUT=/tmp/foo; rm -f $OUT && RUSTC="rustc +nightly-2017-06-01" ; $RUSTC ~/Dev/Rust/foo.rs -O -o $OUT && strip -g $OUT && $RUSTC --version && ls -balh $OUT )
rustc 1.19.0-nightly (e0cc22b4b 2017-05-31)
-rwxr-xr-x 1 pnkfelix pnkfelix 450K Jul 11 18:01 /tmp/foo
% ( OUT=/tmp/foo; rm -f $OUT && RUSTC="rustc +nightly-2017-06-19" ; $RUSTC ~/Dev/Rust/foo.rs -O -o $OUT && strip -g $OUT && $RUSTC --version && ls -balh $OUT )
rustc 1.19.0-nightly (10d7cb44c 2017-06-18)
-rwxr-xr-x 1 pnkfelix pnkfelix 392K Jul 11 18:01 /tmp/foo
% ( OUT=/tmp/foo; rm -f $OUT && RUSTC="rustc +nightly-2017-06-21" ; $RUSTC ~/Dev/Rust/foo.rs -O -o $OUT && strip -g $OUT && $RUSTC --version && ls -balh $OUT )
rustc 1.20.0-nightly (445077963 2017-06-20)
-rwxr-xr-x 1 pnkfelix pnkfelix 416K Jul 11 18:01 /tmp/foo
% ( OUT=/tmp/foo; rm -f $OUT && RUSTC="rustc +nightly-2017-07-07" ; $RUSTC ~/Dev/Rust/foo.rs -O -o $OUT && strip -g $OUT && $RUSTC --version && ls -balh $OUT )
rustc 1.20.0-nightly (696412de7 2017-07-06)
-rwxr-xr-x 1 pnkfelix pnkfelix 453K Jul 11 18:01 /tmp/foo
% ( OUT=/tmp/foo; rm -f $OUT && RUSTC="rustc +nightly-2017-07-11" ; $RUSTC ~/Dev/Rust/foo.rs -O -o $OUT && strip -g $OUT && $RUSTC --version && ls -balh $OUT )
rustc 1.20.0-nightly (bf0a9e0b4 2017-07-10)
-rwxr-xr-x 1 pnkfelix pnkfelix 453K Jul 11 18:02 /tmp/foo
% 
