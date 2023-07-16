
[22:19:20] <nmatsakis> so it seems to me that we can:
[22:19:38] <nmatsakis> (1) implement a prepare_extern_call which is a no-op
[22:19:41] <nmatsakis> (2) implement the lint mode
[22:19:51] <nmatsakis> (3) insert the calls to prepare_extern_call
[22:20:02] <nmatsakis> (4) change prepare_extern_call to do the stack switch and change the call itself to do nothing
[22:20:23] <nmatsakis> -- or rather, remove the wrappers etc
[22:20:37] <nmatsakis> (5) change prepare_extern_call to just reserve a chunk of memory
[22:20:49] <nmatsakis> (6) have a beer
[22:20:59] <nmatsakis> by which I mean: implement a painful and annoying deriving mode
[22:21:03] <nmatsakis> does that sound about right?
[22:23:05] <brson> about, but you may hit problems with those reentrant crust functions between 4 & 5 when prepare_extern_call tries to switch stacks again
[22:23:28] <nmatsakis> right, so prepare_extern_call should reproduce the current smart behavior
[22:23:30] <nmatsakis> in its first iteration
[22:24:10] <brson> that behavior probably depends in some way on the assumption that crust functions do a stack switch, which will no longer be true, so it may just need to change
[22:24:32] <brson> or it may just work :)
[22:26:33] <nmatsakis> ok, this sounds like a plan.
[22:26:39] <nmatsakis> we'll see how it goes :)
