
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os(32) }', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/result.rs:775
stack backtrace:
   1:     0x5555559058f9 - sys::backtrace::write::h36825a208b9947f11fD
   2:     0x55555590d1f2 - panicking::on_panic::h138c9fe8d2c9cb42KvJ
   3:     0x5555558e9c53 - rt::unwind::begin_unwind_inner::h8e4b45288b41856f1bJ
   4:     0x5555558ea011 - rt::unwind::begin_unwind_fmt::h5c0935071a473332CaJ
   5:     0x55555590cc37 - rust_begin_unwind
   6:     0x55555593a3a4 - panicking::panic_fmt::h730a3fc241d0fea6N0A
   7:     0x55555567ac3a - result::Result<T, E>::unwrap::h16844723464354635204
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/macros.rs:27
   8:     0x55555566e125 - resp::respond::h17579872f79925afOAa
                        at /home/mike/.cargo/git/checkouts/joist_rr-e2971a27a9232c5b/master/src/resp.rs:49
   9:     0x55555567b42d - resp::respond_basic::h63c67eef97e61e04YFa
                        at /home/mike/.cargo/git/checkouts/joist_rr-e2971a27a9232c5b/master/src/resp.rs:98
  10:     0x5555555d3851 - pages::home::hb71384c0c9d556c8eqb
                        at src/pages/mod.rs:77
  11:     0x5555555cf1f6 - dispatch::dispatch::h933c39cf4a86b785t1a
                        at src/dispatch.rs:51
  12:     0x5555555b9c3f - handler::JoistHandler::handle_inner::h463e17a7bbf2f736nVa
                        at src/handler.rs:175
  13:     0x5555555b4d2b - handler::JoistHandler.Handler::handle::hc46b7a6b93351cedcQa
                        at src/handler.rs:62
  14:     0x55555562542e - server::handle_connection::h2378902348759844952
                        at /home/mike/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.3.5/src/server/mod.rs:171
  15:     0x55555562350b - server::Server<'a, H, L>::with_listener::closure.11787
                        at /home/mike/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.3.5/src/server/mod.rs:105
  16:     0x55555563cf2e - server::listener::spawn_with::closure.12450
                        at /home/mike/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.3.5/src/server/listener.rs:49
  17:     0x55555563cd11 - thunk::Thunk<'a, (), R>::new::closure.12445
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thunk.rs:27
  18:     0x55555563cc43 - thunk::F.Invoke<A, R>::invoke::h18175814957297039224
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thunk.rs:54
  19:     0x55555561cd4c - thunk::Thunk<'a, A, R>::invoke::h8298915138473532031
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thunk.rs:41
  20:     0x55555561c418 - thread::Builder::spawn_inner::closure.11593
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/mod.rs:354
  21:     0x55555561c3ae - rt::unwind::try::try_fn::__rust_abi::h12818367128688919522
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/rt/unwind.rs:139
  22:     0x55555561c349 - rt::unwind::try::try_fn::h12818367128688919522
  23:     0x555555911498 - rust_try_inner
  24:     0x555555911485 - rust_try
  25:     0x55555561b803 - rt::unwind::try::h11208916803606304307
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/rt/unwind.rs:125
  26:     0x55555561b456 - thread::Builder::spawn_inner::closure.11488
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/mod.rs:354
  27:     0x55555561d269 - thunk::Thunk<'a, (), R>::new::closure.11609
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thunk.rs:27
  28:     0x55555561d1ad - thunk::F.Invoke<A, R>::invoke::h12196283279245811297
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thunk.rs:54
  29:     0x55555590bfd8 - sys::thread::create::thread_start::hdf801f731196670aM5H
  30:     0x7ffff6c8c223 - start_thread
  31:     0x7ffff7b2b5ec - __clone
  32:                0x0 - <unknown>
[New Thread 0x7fffdfbff700 (LWP 13199)]
[New Thread 0x7fffdc2d2700 (LWP 13204)]
[New Thread 0x7fffdf5fc700 (LWP 13203)]
[New Thread 0x7fffdf7fd700 (LWP 13202)]
[New Thread 0x7fffdf9fe700 (LWP 13201)]
[New Thread 0x7fffd79fe700 (LWP 13200)]
[New Thread 0x7ffff43ff700 (LWP 13198)]
[New Thread 0x7ffff57f0700 (LWP 13197)]
[New Thread 0x7ffff59f1700 (LWP 13191)]
[New Thread 0x7ffff5bf2700 (LWP 13190)]
[New Thread 0x7ffff5df3700 (LWP 13189)]

Program received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffdfbff700 (LWP 13199)]
0x0000555555930f58 in je_arena_dalloc_bin_locked ()
(gdb) bt
#0  0x0000555555930f58 in je_arena_dalloc_bin_locked ()
#1  0x00005555559285d5 in tcache_destroy ()
#2  0x0000555555928ed2 in je_tcache_cleanup ()
#3  0x000055555591d50d in je_tsd_cleanup ()
#4  0x00007ffff6c8c072 in __nptl_deallocate_tsd () from /lib64/libpthread.so.0
#5  0x00007ffff6c8c237 in start_thread () from /lib64/libpthread.so.0
#6  0x00007ffff7b2b5ed in clone () from /lib64/libc.so.6
(gdb)
