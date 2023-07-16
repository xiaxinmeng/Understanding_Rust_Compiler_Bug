 irc
Apr 29 23:20:47 <SiegeLord> huon, what do you think of this: https://github.com/mozilla/rust/issues/13663
Apr 29 23:22:09 <huon>  SiegeLord: I thought about it when creating the interface
Apr 29 23:22:40 <huon>  SiegeLord: I thought expressing the true seed type was nice
Apr 29 23:23:02 <huon>  SiegeLord: but the non-uniformity is a little annoying
Apr 29 23:23:13 <huon>  SiegeLord: so... I guess I'm kinda ok with it?
Apr 29 23:23:15 <SiegeLord> Not all RNGs actually implement that trait with the true seed
Apr 29 23:23:45 <huon>  that's a "true seed" for those RNGs
Apr 29 23:24:03 <huon>  i.e. it's legitimate them to see them with partial data
Apr 29 23:24:09 <huon>  *seed
Apr 29 23:24:15 <SiegeLord> I'd think the same would work for XorShift one
Apr 29 23:24:20 <huon>  I guess?
Apr 29 23:24:27 <SiegeLord> The only dissallowed value is all 0's for it
Apr 29 23:25:31 <huon>  I think the seed type would have to be u8
Apr 29 23:25:42 <huon>  hm, actually that's a lie
Apr 29 23:26:17 <huon>  SiegeLord: using a uniform seed type possibly introduces endianness issues
Apr 29 23:26:26 <huon>  (but it's late so I'm not thinking quickly atm...)
Apr 29 23:26:41 <SiegeLord> That's a good point...
Apr 29 23:26:48 <SiegeLord> I'll think about it some more...
Apr 29 23:27:25 <SiegeLord> I just want to be able to pass the seed as a command line argument and it seems... odd for me to create a shim between that number and these implementation-specific values
