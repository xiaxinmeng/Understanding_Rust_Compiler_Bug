
    graydon there's a runtime call called "unsupervise", yes?
    brson   yes
    graydon what happens if we unsupervise the root?
    -->|    nettok (quassel@D59D07A8.37B61F3F.59131532.IP) has joined #rust
    brson   I think the process ends successfully, though I don't know that we have tests for that
    graydon ok. what happens if you join an unsupervised task that has crashed?
    graydon failed I mean
    graydon I'm trying to tease apart the semantics
    brson   you get a message saying that it failed
    -->|    nmatsakis (nmatsakis@E58334A4.A3E4B789.2321E71E.IP) has joined #rust
    graydon we want the runtime to exit(1) but we don't want it to panic and say "oh no, surprise failure!"
    graydon but we *do* want it to say "oh no, surprise failure" when there's an unexpected (otherwise-unreported) failure
    graydon so I'm suggesting that session::error(...) does { stderr.write(formatted error); rt::unsupervise(); fail; }
    graydon and the runtime sees the root failing, in an unsupervised state, and says "well, unsupervised means I don't panic about the error, but since it failed and it's the root I'll exit(1) because it really did not succeed and it'd be a bit superficial of me to call a failing root task exit(0)-status"
    graydon clearer?
    graydon (plausible?)
    |<--    kevina has left moznet (Quit: Leaving.)
    brson   Yes, it's clear. It's strange to me that unsupervise has special meaning to the root task, where failure still propagates, but doesn't log anything.
    brson   what if there are other tasks running still when the unsupervised root task fails?
    brson   what if some random unsupervised task has become the root task (because the root task died) and it's failure causes the process to die?
    graydon good questions, all
    graydon I don't know
    graydon another possibility is to have a set_exit_status(...) call?
    graydon or an rt wrapper that permits calling exit(int) and unwinds properly anyways?
    graydon (assuming we *should* unwind-on-exit? that itself is an interesting question; maybe not?)
    brson   oh yeah. we can just call sys::exit and kablooey everything
    brson   that seems reasonable for rustc
    brson   (not unwinding)
    graydon maybe
    graydon I mean, a destructor is a destructor
    graydon if there's a file handle open, I dunno, some OSs complain
    graydon I don't remember if any dtors run in C++. maybe just the global statics.
    brson   I'm going to add sys::set_exit_status and just make it set some global value in the kernel
