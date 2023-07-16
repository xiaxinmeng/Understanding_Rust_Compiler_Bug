plain
2019-12-06T17:27:51.5670962Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-06T17:27:51.5869751Z ##[command]git config gc.auto 0
2019-12-06T17:27:51.5956255Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-06T17:27:51.6014478Z ##[command]git config --get-all http.proxy
2019-12-06T17:27:51.6179113Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67096/merge:refs/remotes/pull/67096/merge
---
2019-12-06T19:09:51.1907814Z error: Unable to retrieve "http://www.cs.rice.edu/%7Eyguo/pubs/PID824943.pdf": http://www.cs.rice.edu/%7Eyguo/pubs/PID824943.pdf: error trying to connect: failed to lookup address information: Temporary failure in name resolution
2019-12-06T19:09:51.1909202Z 
2019-12-06T19:09:51.1910643Z     ┌── appendix/bibliography.md:27:3 ───
2019-12-06T19:09:51.1911217Z     │
2019-12-06T19:09:51.1912727Z  27 │ * [Work-first and help-first scheduling policies for async-finish task parallelism](http://www.cs.rice.edu/%7Eyguo/pubs/PID824943.pdf) - More general than fully-strict work stealing
2019-12-06T19:09:51.1913952Z     │
2019-12-06T19:09:51.1914124Z 
2019-12-06T19:09:51.1977625Z Error: One or more incorrect links
2019-12-06T19:09:51.1977884Z 
---
2019-12-06T19:53:36.0095875Z 
2019-12-06T19:53:36.0096298Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-12-06T19:53:36.0096484Z commit another update.
2019-12-06T19:53:36.0096601Z 
2019-12-06T19:53:36.0096917Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-12-06T19:53:36.0097312Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-12-06T19:53:36.0097481Z proper steps.
2019-12-06T19:53:36.0097621Z   local time: Fri Dec  6 19:53:35 UTC 2019
2019-12-06T19:53:36.0097660Z   network time: Fri, 06 Dec 2019 19:53:35 GMT
2019-12-06T19:53:36.0097871Z == end clock drift check ==
2019-12-06T19:53:36.3823113Z 
2019-12-06T19:53:36.3823113Z 
2019-12-06T19:53:36.3923678Z ##[error]Bash exited with code '3'.
2019-12-06T19:53:36.3956903Z ##[section]Starting: Checkout
2019-12-06T19:53:36.3959185Z ==============================================================================
2019-12-06T19:53:36.3959265Z Task         : Get sources
2019-12-06T19:53:36.3959313Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
