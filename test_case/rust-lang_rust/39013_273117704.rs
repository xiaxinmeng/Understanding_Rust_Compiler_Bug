
touch tmp/check-stage2-T-mipsel-unknown-linux-gnu-H-mipsel-unknown-linux-gnu-doc-book-concurrency.ok.start_time
LD_LIBRARY_PATH=/<<BUILDDIR>>/rustc-1.14.0+dfsg1/mipsel-unknown-linux-gnu/stage2/lib:/usr/lib/llvm-3.9/lib:$LD_LIBRARY_PATH mipsel-unknown-linux-gnu/stage2/bin/rustdoc --cfg dox --test /<<BUILDDIR>>/rustc-1.14.0
+dfsg1/src/doc/book/concurrency.md --test-args "" && touch -r tmp/check-stage2-T-mipsel-unknown-linux-gnu-H-mipsel-unknown-linux-gnu-doc-book-concurrency.ok.start_time tmp/check-stage2-T-mipsel-unknown-linux-gnu
-H-mipsel-unknown-linux-gnu-doc-book-concurrency.ok && rm tmp/check-stage2-T-mipsel-unknown-linux-gnu-H-mipsel-unknown-linux-gnu-doc-book-concurrency.ok.start_time

running 13 tests
test _0 ... ok
test _1 ... ok
test _2 ... ignored
test _3 ... ok
test _4 ... ignored
test _5 ... ignored
test _6 ... ignored
test _12 ... ok
test _8 ... ignored
test _7 ... ok
test _9 ... ok
test _10 has been running for over 60 seconds
test _11 has been running for over 60 seconds
E: Build killed with signal TERM after 150 minutes of inactivity
--------------------------------------------------------------------------------
Build finished at 2017-01-17T02:01:53Z
