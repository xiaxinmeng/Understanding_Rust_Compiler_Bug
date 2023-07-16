plain
2019-12-05T09:53:03.1860311Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-05T09:53:03.2040215Z ##[command]git config gc.auto 0
2019-12-05T09:53:03.2121138Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-05T09:53:03.2177017Z ##[command]git config --get-all http.proxy
2019-12-05T09:53:03.2292094Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67039/merge:refs/remotes/pull/67039/merge
---
2019-12-05T09:57:52.3173030Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-05T09:57:52.3189641Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-05T09:57:52.6677370Z    Compiling cc v1.0.47
2019-12-05T09:57:52.6677765Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-05T09:57:59.6086923Z error[E0599]: no method named `partial_cmp` found for type `<P as ops::deref::Deref>::Target` in the current scope
2019-12-05T09:57:59.6088193Z    --> src/libcore/pin.rs:437:18
2019-12-05T09:57:59.6088882Z     |
2019-12-05T09:57:59.6089385Z 437 |         (**self).partial_cmp(other)
2019-12-05T09:57:59.6090335Z     |                  ^^^^^^^^^^^ method not found in `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6091555Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2019-12-05T09:57:59.6091555Z     = note: the method `partial_cmp` exists but the following trait bounds were not satisfied:
2019-12-05T09:57:59.6092033Z             `&mut <P as ops::deref::Deref>::Target : iter::traits::iterator::Iterator`
2019-12-05T09:57:59.6092525Z     = help: items from traits can only be used if the trait is implemented and in scope
2019-12-05T09:57:59.6093033Z     = note: the following traits define an item `partial_cmp`, perhaps you need to implement one of them:
2019-12-05T09:57:59.6093489Z             candidate #1: `cmp::PartialOrd`
2019-12-05T09:57:59.6093916Z             candidate #2: `iter::traits::iterator::Iterator`
2019-12-05T09:57:59.6094140Z 
2019-12-05T09:57:59.6137917Z error[E0369]: binary operation `<` cannot be applied to type `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6138530Z    --> src/libcore/pin.rs:441:16
2019-12-05T09:57:59.6138995Z     |
2019-12-05T09:57:59.6139452Z 441 |         **self < **other
2019-12-05T09:57:59.6140160Z     |         ------ ^ ------- <Q as ops::deref::Deref>::Target
2019-12-05T09:57:59.6140634Z     |         |
2019-12-05T09:57:59.6141089Z     |         <P as ops::deref::Deref>::Target
2019-12-05T09:57:59.6141520Z     |
2019-12-05T09:57:59.6142493Z     = note: an implementation of `std::cmp::PartialOrd` might be missing for `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6142728Z 
2019-12-05T09:57:59.6185436Z error[E0369]: binary operation `<=` cannot be applied to type `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6189681Z    --> src/libcore/pin.rs:445:16
2019-12-05T09:57:59.6190274Z     |
2019-12-05T09:57:59.6190741Z 445 |         **self <= **other
2019-12-05T09:57:59.6192199Z     |         ------ ^^ ------- <Q as ops::deref::Deref>::Target
2019-12-05T09:57:59.6193487Z     |         |
2019-12-05T09:57:59.6193981Z     |         <P as ops::deref::Deref>::Target
2019-12-05T09:57:59.6194401Z     |
2019-12-05T09:57:59.6194887Z     = note: an implementation of `std::cmp::PartialOrd` might be missing for `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6195386Z 
2019-12-05T09:57:59.6274654Z error[E0369]: binary operation `>` cannot be applied to type `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6275499Z    --> src/libcore/pin.rs:449:16
2019-12-05T09:57:59.6275882Z     |
2019-12-05T09:57:59.6276275Z 449 |         **self > **other
2019-12-05T09:57:59.6276926Z     |         ------ ^ ------- <Q as ops::deref::Deref>::Target
2019-12-05T09:57:59.6277688Z     |         |
2019-12-05T09:57:59.6281318Z     |         <P as ops::deref::Deref>::Target
2019-12-05T09:57:59.6281921Z     |
2019-12-05T09:57:59.6282344Z     = note: an implementation of `std::cmp::PartialOrd` might be missing for `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6282490Z 
2019-12-05T09:57:59.6286578Z error[E0369]: binary operation `>=` cannot be applied to type `<P as ops::deref::Deref>::Target`
2019-12-05T09:57:59.6287185Z    --> src/libcore/pin.rs:453:16
2019-12-05T09:57:59.6287518Z     |
2019-12-05T09:57:59.6288036Z 453 |         **self >= **other
2019-12-05T09:57:59.6288705Z     |         ------ ^^ ------- <Q as ops::deref::Deref>::Target
2019-12-05T09:57:59.6289085Z     |         |
2019-12-05T09:57:59.6289450Z     |         <P as ops::deref::Deref>::Target
2019-12-05T09:57:59.6289754Z     |
2019-12-05T09:57:59.6290181Z     = note: an implementation of `std::cmp::PartialOrd` might be missing for `<P as ops::deref::Deref>::Target`
2019-12-05T09:58:00.3020793Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-05T09:58:01.7475426Z    Compiling libc v0.2.64
2019-12-05T09:58:02.4883004Z error: aborting due to 5 previous errors
2019-12-05T09:58:02.4883196Z 
---
2019-12-05T09:58:02.5960635Z   local time: Thu Dec  5 09:58:02 UTC 2019
2019-12-05T09:58:02.8851231Z   network time: Thu, 05 Dec 2019 09:58:02 GMT
2019-12-05T09:58:02.8854952Z == end clock drift check ==
2019-12-05T09:58:16.0795886Z 
2019-12-05T09:58:16.0887342Z ##[error]Bash exited with code '1'.
2019-12-05T09:58:16.0911851Z ##[section]Starting: Checkout
2019-12-05T09:58:16.0913422Z ==============================================================================
2019-12-05T09:58:16.0913471Z Task         : Get sources
2019-12-05T09:58:16.0913509Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
