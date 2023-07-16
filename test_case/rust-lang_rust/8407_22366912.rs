
16:43 < bnoordhuis> brson: oh, that's probably fedor's fsevents thread
16:46 < bnoordhuis> brson: yes, it's used to drive the CFRunLoop that in turn powers fsevents
16:47 < bnoordhuis> kind of lame that it gets created unconditionally, i wonder why i didn't notice that during review...
16:47 -!- c4milo [~c4milo@207-38-136-210.c3-0.avec-ubr2.nyr-avec.ny.cable.rcn.com] has joined #libuv
16:48 < brson> lazy initialization of that thread would be sweet :)
16:49 < bnoordhuis> yeah. i'll file an issue
16:50 <@isaacs> bnoordhuis: so... yeah.  we should probably leave it as-is.
16:51 < bnoordhuis> isaacs: 'leave as-is' meaning?
16:52 <@isaacs> bnoordhuis: keep as utf8
