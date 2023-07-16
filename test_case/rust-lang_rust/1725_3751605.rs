
16:55 <@graydon> brson: actually yeah, my bet is on the WaitForSingleObject(thread, INFINITE) in the bottom of rust_timer.cpp
16:55 <@graydon> just delete that file and any uses of it. it's a disaster and we'll do better once we have Real Timers And Stuff from uv
16:55 <@graydon> I'm ashamed to have written it
