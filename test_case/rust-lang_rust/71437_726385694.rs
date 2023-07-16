
2019-07-18 [10:58:45] <jrtc27> cbmuser: problem seems to be delay slots
2019-07-18 [10:59:01] <jrtc27> they break on zzz() calls (random function name just to act as a barrier)
2019-07-18 [10:59:20] <jrtc27> but the call instruction has a delay slot that does the final thing
2019-07-18 [10:59:33] <jrtc27> e.g. in lexical-scope-in-for-loop.rs, it stores the updated value to the stack
2019-07-18 [11:00:08] <jrtc27> and there are two stack slots in use (one for x in the first half of the loop, one for x in the second half)
2019-07-18 [11:00:21] <jrtc27> so the first time round the loop the stack slots are garbage
2019-07-18 [11:00:26] <jrtc27> second time, you get the first time's values
2019-07-18 [11:00:36] <jrtc27> third time you get the second time's values
2019-07-18 [11:01:13] <jrtc27> and outside the loop you get garbage again because the final store for the stack slot outside the loop is in the delay slot
2019-07-18 [11:02:04] <cbmuser> ah
2019-07-18 [11:02:25] <jrtc27> uh wait no actually the final time is fine, it managed to already do the store by then
2019-07-18 [11:02:30] <cbmuser> is sparc64 currently the only architecture in Rust that has delay slots?
2019-07-18 [11:02:35] <jrtc27> no, mips does
2019-07-18 [11:02:44] <jrtc27> as does powerpc
2019-07-18 [11:03:15] <jrtc27> hppa and sh4 do too I think but not relevant
2019-07-18 [11:03:25] <jrtc27> now I should check what gcc does
2019-07-18 [11:04:31] <jrtc27> there are two things I think would be fixes: 1. break in the delay slot not the call itself (though for me that actually feels like the wrong thing to do because if the source is a function call you really want to see the call instruction) 2. tweak debugging info so it says that the value is in a register
