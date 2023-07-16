plain
2020-01-08T03:32:23.9570631Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T03:32:23.9578219Z ##[command]git config gc.auto 0
2020-01-08T03:32:23.9580011Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T03:32:23.9581495Z ##[command]git config --get-all http.proxy
2020-01-08T03:32:23.9583411Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65083/merge:refs/remotes/pull/65083/merge
---
2020-01-08T03:53:06.7146001Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-01-08T03:53:06.7160046Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-08T03:53:07.0272725Z    Compiling cc v1.0.49
2020-01-08T03:53:07.0279292Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-01-08T03:53:10.7981938Z error: use of deprecated item 'intrinsics::init::T': superseded by MaybeUninit, removal planned
2020-01-08T03:53:10.7982450Z     |
2020-01-08T03:53:10.7982656Z 742 |     pub fn init<T>() -> T;
2020-01-08T03:53:10.7982853Z     |                         ^
2020-01-08T03:53:10.7983026Z     |
2020-01-08T03:53:10.7983026Z     |
2020-01-08T03:53:10.7983240Z     = note: `-D deprecated` implied by `-D warnings`
2020-01-08T03:53:10.7983272Z 
2020-01-08T03:53:10.7997863Z error: use of deprecated item 'intrinsics::uninit::T': superseded by MaybeUninit, removal planned
2020-01-08T03:53:10.7998318Z     |
2020-01-08T03:53:10.7998562Z 759 |     pub fn uninit<T>() -> T;
2020-01-08T03:53:10.7998781Z     |                           ^
2020-01-08T03:53:10.7998809Z 
---
2020-01-08T03:53:18.8225151Z   local time: Wed Jan  8 03:53:18 UTC 2020
2020-01-08T03:53:19.0895053Z   network time: Wed, 08 Jan 2020 03:53:19 GMT
2020-01-08T03:53:19.0903639Z == end clock drift check ==
2020-01-08T03:53:20.4394619Z 
2020-01-08T03:53:20.4480290Z ##[error]Bash exited with code '1'.
2020-01-08T03:53:20.4627749Z ##[section]Starting: Checkout
2020-01-08T03:53:20.4629081Z ==============================================================================
2020-01-08T03:53:20.4629121Z Task         : Get sources
2020-01-08T03:53:20.4629176Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
