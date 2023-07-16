

../src/libstd/rt/unwind/gcc.rs:231:38: 231:43 error: use of moved
value: `state` [E0382]
../src/libstd/rt/unwind/gcc.rs:231
__gcc_personality_v0(state, ue_header, context)
                                                                        ^~~~~
../src/libstd/rt/unwind/gcc.rs:225:13: 225:18 note: `state` moved here
because it has type `rt::libunwind::_Unwind_State`, which is
non-copyable
../src/libstd/rt/unwind/gcc.rs:225         if (state as c_int &
uw::_US_ACTION_MASK as c_int)
                                               ^~~~~
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/stamp.std]
Error 101
make: *** Waiting for unfinished jobs....

