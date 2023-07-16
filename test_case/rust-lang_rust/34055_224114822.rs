
15:52 <~acrichto> brson: yeah rustbuild osx works (gated)
15:52 <~acrichto> you're right about -stdlib though, that's the flag not 
                  DEPLOYMENT_VERSION
15:52 <~acrichto> and it all comes back...
15:52 <~acrichto> brson: if you mirror the rustbuild cmake exactly it should in 
                  theory work
15:53 <~acrichto> brson: I think it's this -- 
https://github.com/rust-lang/rust/blob/master/src/bootstrap/build/mod.rs#L808
15:53 <~acrichto> you may have already found that...
