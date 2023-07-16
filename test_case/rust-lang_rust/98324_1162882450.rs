plain
........................................................................................ 176/929
.................................................................i...................... 264/929
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/buffered/tests.rs:495:13
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:37:9
...............................F.......F..F.............F.F...F....F.................... 352/929
........................................................................................ 528/929
........................................................................................ 616/929
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:346:28
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:383:27
---
thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:190:9
.................................................
failures:

---- io::buffered::tests::test_buffered_writer_seek stdout ----
thread 'io::buffered::tests::test_buffered_writer_seek' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5
---- io::cursor::tests::bench_write_vec stdout ----
---- io::cursor::tests::bench_write_vec stdout ----
thread 'io::cursor::tests::bench_write_vec' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5
---- io::cursor::tests::bench_write_vec_vectored stdout ----
---- io::cursor::tests::bench_write_vec_vectored stdout ----
thread 'io::cursor::tests::bench_write_vec_vectored' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5
---- io::cursor::tests::test_mem_mut_writer stdout ----
---- io::cursor::tests::test_mem_mut_writer stdout ----
thread 'io::cursor::tests::test_mem_mut_writer' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5
---- io::cursor::tests::test_mem_writer stdout ----
---- io::cursor::tests::test_mem_writer stdout ----
thread 'io::cursor::tests::test_mem_writer' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5
---- io::cursor::tests::test_seekable_mem_writer stdout ----
---- io::cursor::tests::test_seekable_mem_writer stdout ----
thread 'io::cursor::tests::test_seekable_mem_writer' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5
---- io::cursor::tests::vec_seek_past_end stdout ----
---- io::cursor::tests::vec_seek_past_end stdout ----
thread 'io::cursor::tests::vec_seek_past_end' panicked at 'assertion failed: vec.capacity() - pos > buf.len()', library/std/src/io/cursor.rs:447:5

failures:
    io::buffered::tests::test_buffered_writer_seek
    io::cursor::tests::bench_write_vec
---
    io::cursor::tests::vec_seek_past_end

test result: FAILED. 921 passed; 7 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.29s

error: test failed, to rerun pass '-p std --lib'
