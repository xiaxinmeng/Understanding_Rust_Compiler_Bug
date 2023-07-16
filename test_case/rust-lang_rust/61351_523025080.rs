plain
2019-08-20T12:34:39.0649996Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T12:34:39.0848150Z ##[command]git config gc.auto 0
2019-08-20T12:34:39.0930362Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T12:34:39.0987063Z ##[command]git config --get-all http.proxy
2019-08-20T12:34:39.1150797Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61351/merge:refs/remotes/pull/61351/merge
---
2019-08-20T12:35:16.3396347Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T12:35:16.3396383Z 
2019-08-20T12:35:16.3396624Z   git checkout -b <new-branch-name>
2019-08-20T12:35:16.3396657Z 
2019-08-20T12:35:16.3396731Z HEAD is now at 02cf7607a Merge df80a3b2fce25a309415e1acb4edc9bc0196da97 into 14890954ce17c44d944eda988c5a64bb4c5ec9eb
2019-08-20T12:35:16.3551119Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T12:35:16.3553939Z ==============================================================================
2019-08-20T12:35:16.3553998Z Task         : Bash
2019-08-20T12:35:16.3554061Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T12:45:09.1165681Z    Compiling rand v0.6.1
2019-08-20T12:45:09.5423194Z     Checking num_cpus v1.8.0
2019-08-20T12:45:12.1445226Z     Checking tempfile v3.0.5
2019-08-20T12:45:14.2079748Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-20T12:45:17.3480378Z error[E0277]: can't compare `&std::string::String` with `str`
2019-08-20T12:45:17.3482197Z     |
2019-08-20T12:45:17.3482197Z     |
2019-08-20T12:45:17.3482912Z 674 |             cfgs: cfgs.iter().filter(|cfg| cfg != "rustdoc").collect::<Vec<_>>(),
2019-08-20T12:45:17.3483888Z     |                                                ^^ no implementation for `&std::string::String == str`
2019-08-20T12:45:17.3484537Z     |
2019-08-20T12:45:17.3485170Z     = help: the trait `std::cmp::PartialEq<str>` is not implemented for `&std::string::String`
2019-08-20T12:45:17.3485839Z     = note: required because of the requirements on the impl of `std::cmp::PartialEq<&str>` for `&&std::string::String`
2019-08-20T12:45:17.5618630Z error[E0308]: mismatched types
2019-08-20T12:45:17.5619890Z    --> src/librustdoc/test.rs:674:19
2019-08-20T12:45:17.5620606Z     |
2019-08-20T12:45:17.5620606Z     |
2019-08-20T12:45:17.5621478Z 674 |             cfgs: cfgs.iter().filter(|cfg| cfg != "rustdoc").collect::<Vec<_>>(),
2019-08-20T12:45:17.5622449Z     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::string::String`, found reference
2019-08-20T12:45:17.5623952Z     = note: expected type `std::vec::Vec<std::string::String>`
2019-08-20T12:45:17.5624712Z                found type `std::vec::Vec<&std::string::String>`
2019-08-20T12:45:17.5625092Z 
2019-08-20T12:45:17.6107003Z error: aborting due to 2 previous errors
---
2019-08-20T12:45:17.6646326Z == clock drift check ==
2019-08-20T12:45:18.4803940Z   local time: Tue Aug 20 12:45:17 UTC 2019
2019-08-20T12:45:18.4804063Z   network time: Tue, 20 Aug 2019 12:45:17 GMT
2019-08-20T12:45:18.4804117Z == end clock drift check ==
2019-08-20T12:45:19.4574913Z ##[error]Bash exited with code '1'.
2019-08-20T12:45:19.4631492Z ##[section]Starting: Checkout
2019-08-20T12:45:19.4633617Z ==============================================================================
2019-08-20T12:45:19.4633680Z Task         : Get sources
2019-08-20T12:45:19.4633753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
