
#0  __pthread_kill_implementation (threadid=<optimized out>, signo=signo@entry=6, no_tid=no_tid@entry=0) at pthread_kill.c:44
#1  0x00007ffff70a0953 in __pthread_kill_internal (signo=6, threadid=<optimized out>) at pthread_kill.c:78
#2  0x00007ffff7051ea8 in __GI_raise (sig=sig@entry=6) at ../sysdeps/posix/raise.c:26
#3  0x00007ffff703b53d in __GI_abort () at abort.c:79
#4  0x00007ffff729a833 in __gnu_cxx::__verbose_terminate_handler () at /usr/src/debug/gcc/gcc/libstdc++-v3/libsupc++/vterminate.cc:95
#5  0x00007ffff72a6d0c in __cxxabiv1::__terminate (handler=<optimized out>) at /usr/src/debug/gcc/gcc/libstdc++-v3/libsupc++/eh_terminate.cc:48
#6  0x00007ffff72a5cca in __cxa_call_terminate (ue_header=0x55555616efb0) at /usr/src/debug/gcc/gcc/libstdc++-v3/libsupc++/eh_call.cc:54
#7  0x00007ffff72a644a in __cxxabiv1::__gxx_personality_v0 (version=<optimized out>, actions=6, exception_class=5138137972254386944, ue_header=<optimized out>, context=0x7fffffffc910)
    at /usr/src/debug/gcc/gcc/libstdc++-v3/libsupc++/eh_personality.cc:688
#8  0x00007ffff7a216e4 in _Unwind_RaiseException_Phase2 (exc=exc@entry=0x55555616efb0, context=context@entry=0x7fffffffc910, frames_p=frames_p@entry=0x7fffffffc818)
    at /usr/src/debug/gcc/gcc/libgcc/unwind.inc:64
#9  0x00007ffff7a2215d in _Unwind_Resume (exc=exc@entry=0x55555616efb0) at /usr/src/debug/gcc/gcc/libgcc/unwind.inc:242
#10 0x000055555560ffa6 in location_completer (ignore=<optimized out>, tracker=..., text=<optimized out>) at ../../gdb/completer.c:766
#11 0x000055555572323a in complete_line_internal_1 (tracker=..., text=<optimized out>, line_buffer=<optimized out>, point=<optimized out>, reason=<optimized out>)
    at ../../gdb/completer.c:1438
#12 0x000055555572618a in complete_line_internal (tracker=..., text=<optimized out>, line_buffer=<optimized out>, point=<optimized out>, reason=<optimized out>) at ../../gdb/completer.c:1457
#13 0x00005555557289ac in gdb_completion_word_break_characters_throw () at ../../gdb/completer.c:1907
#14 gdb_completion_word_break_characters () at ../../gdb/completer.c:1945
#15 0x00007ffff7d921aa in _rl_find_completion_word (fp=fp@entry=0x7fffffffce18, dp=dp@entry=0x7fffffffce1c) at ../complete.c:1091
#16 0x00007ffff7d97f9d in rl_complete_internal (what_to_do=9) at ../complete.c:2027
#17 0x00007ffff7d8f430 in _rl_dispatch_subseq (key=9, map=<optimized out>, got_subseq=0) at ../readline.c:916
#18 0x00007ffff7d8f9c6 in _rl_dispatch (key=<optimized out>, map=<optimized out>) at ../readline.c:860
#19 0x00007ffff7d90248 in readline_internal_char () at ../readline.c:675
#20 0x00007ffff7db0406 in rl_callback_read_char () at ../callback.c:272
#21 0x00005555557d9184 in gdb_rl_callback_read_char_wrapper_noexcept () at ../../gdb/event-top.c:188
#22 0x00005555557d9304 in gdb_rl_callback_read_char_wrapper (client_data=<optimized out>) at ../../gdb/event-top.c:221
#23 0x00005555557da930 in stdin_event_handler (error=<optimized out>, client_data=0x555555f2c2f0) at ../../gdb/event-top.c:541
#24 0x0000555555b5be2e in gdb_wait_for_event (block=<optimized out>) at ../gdbsupport/../../gdbsupport/event-loop.cc:716
#25 0x0000555555bad5ca in gdb_do_one_event(int) [clone .constprop.0] (mstimeout=-1) at ../gdbsupport/../../gdbsupport/event-loop.cc:264
#26 0x00005555558d84d5 in start_event_loop () at ../../gdb/main.c:411
#27 captured_command_loop () at ../../gdb/main.c:471
#28 0x0000555555633655 in captured_main (data=0x7fffffffd070) at ../../gdb/main.c:1330
#29 gdb_main (args=0x7fffffffd070) at ../../gdb/main.c:1345
#30 main (argc=<optimized out>, argv=<optimized out>) at ../../gdb/gdb.c:32
