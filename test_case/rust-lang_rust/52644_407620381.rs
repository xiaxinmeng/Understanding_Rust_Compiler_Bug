plain
[01:00:00]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/lib.rs:332:14
[01:00:04]     |
[01:00:04] 332 | #[cfg(test)] extern crate test;
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:13:1
[01:00:04]    |
[01:00:04] 13 | extern crate test;
[01:00:04] 13 | extern crate test;
[01:00:04]    | ^^^^^^^^^^^^^^^^^^
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:15:5
[01:00:04]    |
[01:00:04]    |
[01:00:04] 15 | use self::test::Bencher;
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:18:21
[01:00:04]    |
[01:00:04]    |
[01:00:04] 18 | fn new_drop(b: &mut Bencher) {
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:28:28
[01:00:04]    |
[01:00:04]    |
[01:00:04] 28 | fn new_insert_drop(b: &mut Bencher) {
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:39:30
[01:00:04]    |
[01:00:04]    |
[01:00:04] 39 | fn grow_by_insertion(b: &mut Bencher) {
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:57:26
[01:00:04]    |
[01:00:04]    |
[01:00:04] 57 | fn find_existing(b: &mut Bencher) {
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:74:29
[01:00:04]    |
[01:00:04]    |
[01:00:04] 74 | fn find_nonexisting(b: &mut Bencher) {
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]   --> libstd/collections/hash/bench.rs:91:29
[01:00:04]    |
[01:00:04]    |
[01:00:04] 91 | fn hashmap_as_queue(b: &mut Bencher) {
[01:00:04]    |
[01:00:04]    |
[01:00:04]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/collections/hash/bench.rs:110:30
[01:00:04]     |
[01:00:04]     |
[01:00:04] 110 | fn get_remove_insert(b: &mut Bencher) {
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/buffered.rs:970:9
[01:00:04]     |
[01:00:04] 970 |     use test;
[01:00:04] 970 |     use test;
[01:00:04]     |         ^^^^
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]     --> libstd/io/buffered.rs:1330:38
[01:00:04]      |
[01:00:04]      |
[01:00:04] 1330 |     fn bench_buffered_reader(b: &mut test::Bencher) {
[01:00:04]      |
[01:00:04]      |
[01:00:04]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]     --> libstd/io/buffered.rs:1337:38
[01:00:04]      |
[01:00:04]      |
[01:00:04] 1337 |     fn bench_buffered_writer(b: &mut test::Bencher) {
[01:00:04]      |
[01:00:04]      |
[01:00:04]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:280:9
[01:00:04]     |
[01:00:04] 280 |     use test;
[01:00:04] 280 |     use test;
[01:00:04]     |         ^^^^
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:283:33
[01:00:04]     |
[01:00:04]     |
[01:00:04] 283 |     fn bench_read_slice(b: &mut test::Bencher) {
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:291:17
[01:00:04]     |
[01:00:04]     |
[01:00:04] 291 |                 test::black_box(&dst);
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:297:34
[01:00:04]     |
[01:00:04]     |
[01:00:04] 297 |     fn bench_write_slice(b: &mut test::Bencher) {
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:305:17
[01:00:04]     |
[01:00:04]     |
[01:00:04] 305 |                 test::black_box(&wr);
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:311:31
[01:00:04]     |
[01:00:04]     |
[01:00:04] 311 |     fn bench_read_vec(b: &mut test::Bencher) {
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:319:17
[01:00:04]     |
[01:00:04]     |
[01:00:04] 319 |                 test::black_box(&dst);
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:325:32
[01:00:04]     |
[01:00:04]     |
[01:00:04] 325 |     fn bench_write_vec(b: &mut test::Bencher) {
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/io/impls.rs:333:17
[01:00:04]     |
[01:00:04]     |
[01:00:04] 333 |                 test::black_box(&wr);
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]     --> libstd/io/mod.rs:2178:9
[01:00:04]      |
[01:00:04] 2178 |     use test;
[01:00:04] 2178 |     use test;
[01:00:04]      |         ^^^^
[01:00:04]      |
[01:00:04]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]     --> libstd/io/mod.rs:2394:34
[01:00:04]      |
[01:00:04]      |
[01:00:04] 2394 |     fn bench_read_to_end(b: &mut test::Bencher) {
[01:00:04]      |
[01:00:04]      |
[01:00:04]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/num.rs:288:5
[01:00:04]     |
[01:00:04] 288 |     extern crate test;
[01:00:04] 288 |     extern crate test;
[01:00:04]     |     ^^^^^^^^^^^^^^^^^^
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/num.rs:289:9
[01:00:04]     |
[01:00:04]     |
[01:00:04] 289 |     use self::test::Bencher;
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:04] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:04]    --> libstd/num.rs:292:35
[01:00:04]     |
[01:00:04]     |
[01:00:04] 292 |     fn bench_pow_function(b: &mut Bencher) {
[01:00:04]     |
[01:00:04]     |
[01:00:04]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]   --> libstd/collections/hash/bench.rs:21:7
[01:00:06]    |
[01:00:06]    |
[01:00:06] 21 |     b.iter(|| {
[01:00:06]    |
[01:00:06]    |
[01:00:06]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]   --> libstd/collections/hash/bench.rs:31:7
[01:00:06]    |
[01:00:06]    |
[01:00:06] 31 |     b.iter(|| {
[01:00:06]    |
[01:00:06]    |
[01:00:06]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]   --> libstd/collections/hash/bench.rs:50:7
[01:00:06]    |
[01:00:06]    |
[01:00:06] 50 |     b.iter(|| {
[01:00:06]    |
[01:00:06]    |
[01:00:06]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]   --> libstd/collections/hash/bench.rs:66:7
[01:00:06]    |
[01:00:06]    |
[01:00:06] 66 |     b.iter(|| {
[01:00:06]    |
[01:00:06]    |
[01:00:06]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]   --> libstd/collections/hash/bench.rs:83:7
[01:00:06]    |
[01:00:06]    |
[01:00:06] 83 |     b.iter(|| {
[01:00:06]    |
[01:00:06]    |
[01:00:06]    = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]    --> libstd/collections/hash/bench.rs:102:7
[01:00:06]     |
[01:00:06]     |
[01:00:06] 102 |     b.iter(|| {
[01:00:06]     |
[01:00:06]     |
[01:00:06]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:06] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:06]    --> libstd/collections/hash/bench.rs:121:7
[01:00:06]     |
[01:00:06]     |
[01:00:06] 121 |     b.iter(|| {
[01:00:06]     |
[01:00:06]     |
[01:00:06]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]     --> libstd/io/mod.rs:2395:11
[01:00:08]      |
[01:00:08]      |
[01:00:08] 2395 |         b.iter(|| {
[01:00:08]      |
[01:00:08]      |
[01:00:08]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]     --> libstd/io/buffered.rs:1331:11
[01:00:08]      |
[01:00:08]      |
[01:00:08] 1331 |         b.iter(|| {
[01:00:08]      |
[01:00:08]      |
[01:00:08]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]     --> libstd/io/buffered.rs:1338:11
[01:00:08]      |
[01:00:08]      |
[01:00:08] 1338 |         b.iter(|| {
[01:00:08]      |
[01:00:08]      |
[01:00:08]      = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]    --> libstd/io/impls.rs:287:11
[01:00:08]     |
[01:00:08]     |
[01:00:08] 287 |         b.iter(|| {
[01:00:08]     |
[01:00:08]     |
[01:00:08]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]    --> libstd/io/impls.rs:301:11
[01:00:08]     |
[01:00:08]     |
[01:00:08] 301 |         b.iter(|| {
[01:00:08]     |
[01:00:08]     |
[01:00:08]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]    --> libstd/io/impls.rs:315:11
[01:00:08]     |
[01:00:08]     |
[01:00:08] 315 |         b.iter(|| {
[01:00:08]     |
[01:00:08]     |
[01:00:08]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:08] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:08]    --> libstd/io/impls.rs:329:11
[01:00:08]     |
[01:00:08]     |
[01:00:08] 329 |         b.iter(|| {
[01:00:08]     |
[01:00:08]     |
[01:00:08]     = help: add #![feature(test)] to the crate attributes to enable
[01:00:10] error[E0658]: use of unstable library feature 'test' (see issue #27812)
[01:00:10]    --> libstd/num.rs:294:11
[01:00:10]     |
[01:00:10]     |
[01:00:10] 294 |         b.iter(|| {v.iter().fold(0u32, |old, new| old.pow(*new as u32));});
[01:00:10]     |
[01:00:10]     |
[01:00:10]     = help: add #![feature(test)] to the crate attributes to enable
travis_time:end:323ccd20:start=1532484991186400163,finish=1532488605387631505,duration=3614201231342

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2799e568
