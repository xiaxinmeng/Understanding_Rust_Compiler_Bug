
21:52:43 <doener> Talking about rust, we have an open issue about improvintour IR for pointers 
                  that are guaranteed to be non-null. Pointers for sret and inalloca arguments 
                  are already marked as known not to be null. I wonder if there's a way to 
                  mark other pointers as such, or if a new attribute would be required (and if 
                  so, if it would be welcome).
21:53:29 <nlewycky> a new attribute would be required
21:53:34 <nlewycky> last time this came up i made a counter-proposal
21:53:46 <nlewycky> instead of simply "non-null" which only allows us to eliminate "x == null" 
                    checks
21:54:00 <nlewycky> how about "k bytes of ptr are dereferencable"?
21:54:19 <nlewycky> this allows us to do things like remove null checks (can't dereference 
                    null) and also hoist loads out of loops, etc.
21:54:37 <nlewycky> we could also use this for c++ references, for example
21:56:35 <zygoloid> dereferenceable(ptrtoint i8* (getelementptr {ty, i8}* null, 0, 1) to i32) ?
21:57:22 <nlewycky> nope. let's make it an integer.
21:57:32 <zygoloid> that is an integer :)
21:57:57 <zygoloid> apart from my errors in writing IR, that should be sizeof(ty)
21:57:59 <nlewycky> llvm attributes can either by no-argument, string argument or integer 
                    argument, not llvm ir.
21:58:24 <zygoloid> but...but... i wanna use the size of an opaque type ;)
21:58:35 <nlewycky> ...and this is exactly why
21:58:41 <nlewycky> wait
21:58:50 <nlewycky> are you saying you want to compile code that passes forward declared 
                    references around
21:58:57 <zygoloid> yes please
21:58:58 <nlewycky> then resolves the number of dereferencable bytes at lto time?
21:59:03 <zygoloid> :)
21:59:07 <zygoloid> that is what i want
21:59:08  * nlewycky sighs loudly
22:00:24 <zygoloid> nlewycky: but, you know, i'm ok punting on this
22:00:39 <zygoloid> i don't think it matters in practice, because the type must be complete 
                    anywhere you actaully have a use of it
22:00:51 <nlewycky> okay that's what i thought
22:00:57 <nlewycky> i was trying to imagine how this could possibly matter
22:01:00 <zygoloid> so we can just emit dereferenceable(1), and ask LTO to take the max
22:01:46 <nlewycky> if you unconditionally pass a pointer to a callee that is marked 
                    dereferencable(n), your pointer may also be marked deref(n). 
                    implementation goes in functionattrs please.
22:02:19 <zygoloid> nlewycky: i was thinking the same thing :)
22:02:59 <zygoloid> nlewycky: can we mark loads as deref(n) too?
22:03:23 <zygoloid> when i load my reference, i want to express to llvm that the resulting 
                    pointer is dereferenceable
22:03:30 <nlewycky> ahhh
22:03:33  * nlewycky ponders
22:03:57 <zygoloid> how do you express the lifetime of a deref(n)?
22:03:57 <nlewycky> gosh, i guess you can.
22:04:23 <zygoloid> how do i retain deref(n) after inlining?
22:04:29 <nlewycky> er ... no. this is a static marker. you don't get to change it.
22:04:41 <zygoloid> then i can't use it for references, i think?
22:04:52 <nlewycky> huh? you can't rebind a reference
22:04:58 <zygoloid> no, but a reference's lifetime can end
22:04:59 <nlewycky> it's propagated via functionattrs, used by the local optz'ns, discarded by 
                    the inliner.
22:05:35 <zygoloid> ok, so i think this is actually fine
22:06:01 <zygoloid> deref(n) remains true for any use of the pointer, and stops being true at 
                    the last use
22:06:34 <zygoloid> but this gets us into the 'same pointer value' pain
22:07:18 <zygoloid> eg, union { struct X { int &r; } x; struct Y { char &c; } y; };
22:07:55 <zygoloid> if i observe that the pointer is a dereferenceable pointer to 4 bytes (for 
                    the first union member), then i change the active member to y...
22:08:21 <nlewycky> you get to derive minimal safe dereference from the type system
