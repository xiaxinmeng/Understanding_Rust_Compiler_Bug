
[8:54 AM] QuietMisdreavus: question for docs folk: do we want to show #[inline] attributes on functions? https://github.com/rust-lang/rust/pull/54014
[8:54 AM] QuietMisdreavus: i'm leaning toward "no", but weakly enough that i'm seeking other opinions
[9:10 AM] steveklabnik: Hmm
[9:35 AM] imperio: @QuietMisdreavus just to bring into the debate: some other info: some other attributes are there
[9:39 AM] QuietMisdreavus: @imperio right, but those other attribute affect how people use the function in question
[9:40 AM] QuietMisdreavus: imo, #[inline] is an optimization that people generally don't care about
[9:46 AM] imperio: hum ok
[10:49 AM] steveklabnik: i think part of this question is "is removing #[inline] a breaking change?
[10:49 AM] steveklabnik: that is, is it part of the API guarantee?
[10:49 AM] steveklabnik: if it is, it should be documented
[10:50 AM] steveklabnik: if it's not, that's a good argument to not document it
[11:18 AM] imperio: @steveklabnik then I suppose it is
[11:18 AM] QuietMisdreavus: i think the answer depends on who you ask
[11:19 AM] QuietMisdreavus: iirc the compiler does a fair amount of inlining without needing the attribute, but i dunno how much that happens across crates
[11:20 AM] steveklabnik: yeah
[11:20 AM] steveklabnik: we don't have this as part of the stability rfcs, i don't think
[11:20 AM] steveklabnik: :/
[11:20 AM] steveklabnik: either way
[3:16 PM] imperio: @steveklabnik @QuietMisdreavus should we ask to other rust teams? They might have stronger opinions on this
[3:16 PM] imperio: if we can't decide it'll make things simpler :smile:
[3:46 PM] steveklabnik: yeah, most likely, maybe an internals post?
[4:20 PM] imperio: sounds like a good idea :smiley:
[5:23 PM] scottmcm: I don't think performance is ever part of a stability guarantee.  New versions can absolutely regress or improve performance, so I don't think that #[inline] is relevant
[5:23 PM] scottmcm: especially since you could always just make everything #[inline] as a thin wrapper that delegates to something expensive and non-inline underneath
[5:24 PM] scottmcm: I wouldn't want people to start getting bugs on their creates like "I was looking at the docs, and it looks like you only marked 4 of the 5 methods as inline"
[5:25 PM] scottmcm: (Also, if inline needs to be in the docs, things like cold also should be.)
[5:35 PM] Havvy: IMO, if it's doc-worthy that it's inlined, it should be in the prose.
