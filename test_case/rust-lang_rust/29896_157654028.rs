
---- [run-pass-valgrind] run-pass-valgrind/exit-flushes.rs stdout ----

error: test run failed!
status: exit code: 100
command: /usr/bin/valgrind --error-exitcode=100 --fair-sched=try --quiet --soname-synonyms=somalloc=NONE --suppressions=/home/nagisa/Documents/rust/rust/src/etc/x86.supp --tool=memcheck --leak-check=full x86_64-unknown-linux-gnu/test/run-pass-valgrind/exit-flushes.stage1-x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
==28178== 288 bytes in 1 blocks are possibly lost in loss record 2 of 2
==28178==    at 0x4C2A987: calloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==28178==    by 0x4011841: allocate_dtv (in /usr/lib/ld-2.22.so)
==28178==    by 0x401221D: _dl_allocate_tls (in /usr/lib/ld-2.22.so)
==28178==    by 0x5CDB044: pthread_create@@GLIBC_2.2.5 (in /usr/lib/libpthread-2.22.so)
==28178==    by 0x4F4E989: sys::thread::_$LT$impl$GT$::new::h18c8e1383a16a102QFw (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F4BD67: thread::_$LT$impl$GT$::spawn::h8240507308132532220 (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F4AE15: thread::spawn::h8206638966982322642 (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F4341D: process::_$LT$impl$GT$::wait_with_output::read::h16290293560351911718 (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F4042A: process::_$LT$impl$GT$::wait_with_output::h149e8dca7ba9fec96Pm (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F402AD: process::_$LT$impl$GT$::output::_$LT$closure$GT$::closure.38650 (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F40128: result::_$LT$impl$GT$::and_then::and_then::h12745603337970662170 (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178==    by 0x4F40077: process::_$LT$impl$GT$::output::h85147110d45a11184Bm (in /home/nagisa/Documents/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-71b07a99.so)
==28178== 
------------------------------------------
