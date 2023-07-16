
<@pcwalton> brson and I are beginning to think that "headless mode" is not
            really what we want
<@pcwalton> rather, the runtime should just be pluggable
<@pcwalton> and a "headless mode" is just a very simple runtime
<@pcwalton> with some stuff unimplemented, perhaps
<@graydon> "runtime-less" seems like a misnomer, I agree. I'd just aim to tackle
           the things that prevent specific use-cases of rust libraries, one by
           one
<@graydon> I agree we should shrink ours down and simplify / make-optional as
           many parts as practical
<@pcwalton> yes, totally agreed
<@graydon> but I don't see this as a single big-bang project
<@pcwalton> mainly the tricky parts are with stack switching
<@pcwalton> brson and I had ideas there to make stack growth optional
<@pcwalton> but I forget what they were offhand
<@graydon> mhm. it might be that the easiest thing to do there is just make
           stack growth optio... what you said :)
<@graydon> on 64bit it's kinda pointless anyway
<@graydon> I sorta think disabling it is semi-equivalent to having "1:1 mode" in
           the scheduler, or forcing that to be the only mode
<@graydon> maybe not exactly equivalent, as the tasks may still be cheaper than
           threads in terms of allocated kernel resources
