
15:10 <~acrichto> we have two options
15:10 <~acrichto> 1) revert back to binutils 2.25
15:10 <~acrichto> 2) pass two new fancy flags to disable the new behavior
15:10 <~acrichto> so (1) is quite easy, just go build a new container and call it a 
                  day
15:10 <~acrichto> will make rillian unhappy though
15:10 <~acrichto> unfortunately I don't think we can do (2) though
15:11 <~acrichto> passing two new fancy flags will break the rust build on a ton of 
                  platforms presumably which don't understand the flags
15:11 <~acrichto> and the last thing I want to do is to do feature detection here
15:11 <~acrichto> oh and I also don't know how to pass cflags just on the builders
15:11 <~acrichto> so my curret thinking for a plan of action is:
15:11 <~acrichto> a) revert back to binutils 2.25
15:11 <~acrichto> b) switch nightlies to use rustbuild
15:11 <~acrichto> c) update back to 2.27, but pass the fancy flags *only* on the 
                  builder
15:12 <~acrichto> where (c) is possible b/c rustflags is much easier to configure w/ 
                  CFLAGS env vars and whatnot
15:12 <~acrichto> or rather, I trust it a whole lot more
