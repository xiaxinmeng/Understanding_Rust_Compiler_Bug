
$ RUST_BACKTRACE=1 make
cfg: build triple i686-w64-mingw32
cfg: host triples i686-w64-mingw32
cfg: target triples i686-w64-mingw32
cfg: host for i686-w64-mingw32 is i386
cfg: os for i686-w64-mingw32 is w64-mingw32
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind due to its unreliability on this platform
cfg: no llnextgen found, omitting grammar-verification
cfg: disabling doc build (CFG_DISABLE_DOCS)
rustc: i686-w64-mingw32/stage0/bin/rustlib/i686-w64-mingw32/lib/librand
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.1.o:(.rdata+0x2a0): multiple definition of `isaac::IsaacRng.Rng::next_u32::_MSG_FILE_LINE::h5ef920f83fd19327dMb'
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.0.o:(.rdata+0x3f0): first defined here
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.1.o:(.rdata+0x2f0): multiple definition of `isaac::Isaac64Rng.Rng::next_u64::_MSG_FILE_LINE::h5ef920f83fd19327Mgc'
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.0.o:(.rdata+0x440): first defined here
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.2.o:(.rdata+0x170): multiple definition of `isaac::IsaacRng.Rng::next_u32::_MSG_FILE_LINE::h5ef920f83fd19327dMb'
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.0.o:(.rdata+0x3f0): first defined here
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.2.o:(.rdata+0x1c0): multiple definition of `isaac::Isaac64Rng.Rng::next_u64::_MSG_FILE_LINE::h5ef920f83fd19327Mgc'
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.0.o:(.rdata+0x440): first defined here
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.3.o:(.rdata+0x2b30): multiple definition of `isaac::IsaacRng.Rng::next_u32::_MSG_FILE_LINE::h5ef920f83fd19327dMb'
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.0.o:(.rdata+0x3f0): first defined here
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.3.o:(.rdata+0x3ba0): multiple definition of `isaac::Isaac64Rng.Rng::next_u64::_MSG_FILE_LINE::h5ef920f83fd19327Mgc'
i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.0.o:(.rdata+0x440): first defined here
collect2.exe: error: ld returned 1 exit status
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'called `Result::unwrap()` on an `Err` value: couldn't rename path (file not found (OS Error 2: Das System kann die angegebene Datei nicht finden.
); from=i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.o.exe; to=i686-w64-mingw32\stage0\bin\rustlib\i686-w64-mingw32\lib\rand-4e7c5e5c.o)', F:\Dokumente\Coding\rust\rust-lang\src\libcore\result.rs:789

stack backtrace:
   1:   0x8235cc - ZN2rt4init20hfcca88ec6cfbbb64PVqE
   2: 0x6290651c - ZN6unwind18begin_unwind_inner20hbadd90e57916f74csgdE
   3: 0x629062ad - ZN6unwind16begin_unwind_fmt20h35cd0cf23edb2fe5UddE
   4: 0x629060c5 - rust_begin_unwind
   5: 0x62963c4f - ZN7failure8fail_fmt20h6f560916285a7c12r6jE
   6: 0x6fcf1e72 - ZN6driver6driver35OutputFilenames...std..clone..Clone5clone20h92f308eb1ecaa85dMFAE
   7: 0x6fceee19 - ZN4back5write10run_passes20hb458ae00ff4146b2kScE
   8: 0x704ddbd6 - ZN4lint7context11check_crate20h74ddfe7130dd0539a7HE
   9: 0x6fce1109 - ZN8metadata6cstore6CStore21get_used_crate_source20h44586f0b7c3e05a3VvxE
  10: 0x7048a94a - ZN6driver6driver23phase_5_run_llvm_passes20h2f933419587f07ecgkAE
  11: 0x70484285 - ZN6driver6driver13compile_input20h425d923af718a892iQzE
  12: 0x70504dec - ZN6driver7monitor20h65c71efef7bbade2xrEE
  13: 0x70502db4 - ZN6driver7monitor20h65c71efef7bbade2xrEE
  14: 0x6fcf908e - ZN6driver6driver35OutputFilenames...std..clone..Clone5clone20h92f308eb1ecaa85dMFAE
  15: 0x6fcf8fd6 - ZN6driver6driver35OutputFilenames...std..clone..Clone5clone20h92f308eb1ecaa85dMFAE
  16: 0x6aee9d43 - ZN4task15Ops.rt..Runtime4wrap20h0122d5dc6ff0992dVxeE
  17: 0x62905f1c - ZN6unwind8Unwinder9unwinding20hd2e2793548e8061dX4cE
  18: 0x629044fe - ZN4task4Task3run20h2fd13b26e6c6503aAzcE
  19: 0x6aee9bb1 - ZN4task15Ops.rt..Runtime4wrap20h0122d5dc6ff0992dVxeE
  20: 0x62905c07 - ZN4task11BlockedTask14cast_from_uint20hae61d96ed7595d1cdScE
  21: 0x772a9f72 - RtlInitializeExceptionChain

/f/Dokumente/Coding/rust/rust-lang/mk/target.mk:166: recipe for target 'i686-w64-mingw32/stage0/bin/rustlib/i686-w64-mingw32/lib/stamp.rand' failed
make: *** [i686-w64-mingw32/stage0/bin/rustlib/i686-w64-mingw32/lib/stamp.rand] Error 101
