
19:46 < sfackler> brson: do you know if there's any way to get stuff in test/auxiliary to link against the same version of e.g. libsyntax that that version of rustc links 
                  against?
19:46 < sfackler> just doing an "extern mod syntax" seems to link a stage too high
19:46 -!- kimundi [kimundi@moz-403D73FE.dip0.t-ipconnect.de] has quit [Ping timeout]
19:48 <@brson> sfackler: no, there isn't a way. because of the way the stage progression works, rustc is always linked to the previous stage
19:48 <@brson> i can't imagine a reason that would matter though? what odd thing are you trying to do?
19:49 <@brson> if you link to rustc from aux though you'll get a different rustc than the one used to do the compilation
19:49 < sfackler> external syntax extensions! https://github.com/sfackler/rust/blob/28206f335c02ee5fef1eded5ab13dd3b459db839/src/test/auxiliary/macro_crate_test.rs
19:50 -!- kimundi [kimundi@moz-626CD9BD.dip0.t-ipconnect.de] has joined #rust-internals
19:50 <@brson> aha
19:51 < sfackler> I need to use the same libsyntax binary or the dynamic linker fails to match things up properly
19:51 <@brson> that is going to be tricky because of this issue :(
19:51 <@brson> i would like to build the stages in a different way so that they aren't intermixed like this, but there are complications
19:52 <@brson> this is something we need to resolve though
19:52 <@brson> i guess I'd prefer to just bite the bullet and *completely change the build process*
19:52 < sfackler> how does run-pass-fulldeps work? would it be possible to do something like that for auxilliary-fulldeps?
19:53 <@brson> sfackler: no, you would still be getting the wrong libs
19:53 <@brson> the rustc exe basically has its own set of libraries
19:53 <@brson> from the previous stage
19:53 <@brson> you could hack something together to manually point your library to the correct libraries
19:54 <@brson> but that's minimally useful
19:54 < sfackler> yeah
19:54 <@brson> with -L
19:54 -!- kimundi is now known as zz_kimundi
19:55 <@brson> in theory the stage3 rustc should have the same libs as its targets
19:55 <@brson> but stage3 is not used much
19:55 <@brson> and that's not always true, for unknown reasons
19:57 <@brson> i'll file an issue about this
19:58 < sfackler> cool. I'm guessing that this'll block on getting the test infrastructure to a place where it can actually have an rpass test, right?
19:59 -!- canhtak [canhtak@38266F07.7A8E554D.FC900DAF.IP] has quit [Quit: canhtak]
20:00 <@brson> sfackler: does it work manually, but not in a test harness? that would surprise me
20:00 < sfackler> it works off of a full install
20:01 -!- zz_kimundi is now known as kimundi
20:01 <@brson> ok. probably because the symbols for the host bins and target bins are identical
20:01 < sfackler> and i'm guessing that  work at stage1 if I pointed it at the right libsyntax
20:01 < sfackler> *that it'll
20:01 <@brson> does the test work in stage2?
20:02 <@brson> i would expect so, since stage2 is the install stage
20:02 < sfackler> let me check
20:02 < sfackler> it does appear to work at stage2
20:03 <@brson> if it works reliably in stage2 we *might* be able to merge it, but I'm not sure how much we can rely on the stage2 symbols being the same
20:03 < sfackler> is rustc statically linked?
20:04 < sfackler> it should definitely work with the installed version of rustc since rustc and the extension crate will be linking to the exact same library
20:05 <@brson> they won't be linking to the same library. there are always at least two copies of the libs installed
20:05 < sfackler> oh, really?
20:05 <@brson> one in /usr/lib and one in /usr/lib/rustc/$triple/lib
20:05 < sfackler> ah
20:06 <@brson> i would rather the one in /usr/lib didn't exist
20:06 <@brson> it's very compilcated to change the build process though
20:06 < sfackler> yeah, it seems like if you want to link against libsyntax or librustc, you'd always want the same version as rustc's
20:08 <@brson> yes. it's tricky though. the rustc that is compiling your crate that is linking to libsyntax needs to be exactly compatible with the one that built libsyntax, 
               but it itself also needs to link to the libsyntax that it built
20:08 <@brson> bootstrapping is neat
20:09 <@brson> s/neat/hard
