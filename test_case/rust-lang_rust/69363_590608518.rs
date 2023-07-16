plain
2020-02-24T23:20:15.6150160Z ========================== Starting Command Output ===========================
2020-02-24T23:20:15.6154423Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2e0ac405-d279-4122-bc9a-89961649bca7.sh
2020-02-24T23:20:15.6154977Z 
2020-02-24T23:20:15.6160150Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-24T23:20:15.6192619Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-24T23:20:15.6206395Z Task         : Get sources
2020-02-24T23:20:15.6206709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-24T23:20:15.6207005Z Version      : 1.0.0
2020-02-24T23:20:15.6207224Z Author       : Microsoft
---
2020-02-24T23:20:16.6196953Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-24T23:20:16.6202253Z ##[command]git config gc.auto 0
2020-02-24T23:20:16.6206561Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-24T23:20:16.6210701Z ##[command]git config --get-all http.proxy
2020-02-24T23:20:16.6217440Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69363/merge:refs/remotes/pull/69363/merge
---
2020-02-24T23:53:07.1436866Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-24T23:53:10.8764338Z error: type parameter on variant
2020-02-24T23:53:10.8767987Z    --> src/librustc/mir/interpret/mod.rs:7:19
2020-02-24T23:53:10.8768491Z     |
2020-02-24T23:53:10.8769114Z 4   |  / macro_rules! err_unsup {
2020-02-24T23:53:10.8770162Z 5   |  |     ($($tt:tt)*) => {
2020-02-24T23:53:10.8771138Z 6   |  |         $crate::mir::interpret::InterpError::Unsupported(
2020-02-24T23:53:10.8772211Z 7   |  |             $crate::mir::interpret::UnsupportedOpInfo::$($tt)*
2020-02-24T23:53:10.8774397Z     |  |___________________|
2020-02-24T23:53:10.8775024Z     | ||
2020-02-24T23:53:10.8775719Z 8   | ||         )
2020-02-24T23:53:10.8776695Z 9   | ||     };
2020-02-24T23:53:10.8776695Z 9   | ||     };
2020-02-24T23:53:10.8777866Z 10  | || }
2020-02-24T23:53:10.8778594Z     | || -
2020-02-24T23:53:10.8779224Z     | ||_|
2020-02-24T23:53:10.8779901Z     | |  in this expansion of `err_unsup!`
2020-02-24T23:53:10.8780494Z ...   |
2020-02-24T23:53:10.8781411Z 50  | /  macro_rules! throw_unsup {
2020-02-24T23:53:10.8782303Z 51  |        ($($tt:tt)*) => { return Err(err_unsup!($($tt)*).into()) };
2020-02-24T23:53:10.8784155Z 52  | |  }
2020-02-24T23:53:10.8784949Z     | |__- in this expansion of `throw_unsup!`
2020-02-24T23:53:10.8785492Z ...
2020-02-24T23:53:10.8785492Z ...
2020-02-24T23:53:10.8786323Z 226 | |          static DECODER_SESSION_ID: AtomicU32 = AtomicU32::new(0);
2020-02-24T23:53:10.8787573Z 227 | |          let counter = DECODER_SESSION_ID.fetch_add(1, Ordering::SeqCst);
2020-02-24T23:53:10.8789035Z     | 
2020-02-24T23:53:10.8789726Z    ::: src/librustc/mir/interpret/pointer.rs:227:13
2020-02-24T23:53:10.8790229Z     |
2020-02-24T23:53:10.8790229Z     |
2020-02-24T23:53:10.8790991Z 227 |                throw_unsup!(PointerOutOfBounds { ptr: self.erase_tag(), msg, allocation_size })
2020-02-24T23:53:10.8792833Z     |
2020-02-24T23:53:10.8792833Z     |
2020-02-24T23:53:10.8793444Z     = note: `-D type-param-on-variant-ctor` implied by `-D warnings`
2020-02-24T23:53:18.1916604Z error: aborting due to previous error
2020-02-24T23:53:18.1917633Z 
2020-02-24T23:53:18.2079636Z error: could not compile `rustc`.
2020-02-24T23:53:18.2080919Z warning: build failed, waiting for other jobs to finish...
---
2020-02-24T23:53:37.3977420Z   local time: Mon Feb 24 23:53:37 UTC 2020
2020-02-24T23:53:37.6887470Z   network time: Mon, 24 Feb 2020 23:53:37 GMT
2020-02-24T23:53:37.6891041Z == end clock drift check ==
2020-02-24T23:53:38.1089379Z 
2020-02-24T23:53:38.1153862Z ##[error]Bash exited with code '1'.
2020-02-24T23:53:38.1176779Z ##[section]Finishing: Run build
2020-02-24T23:53:38.1220070Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-24T23:53:38.1225181Z Task         : Get sources
2020-02-24T23:53:38.1225537Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-24T23:53:38.1225839Z Version      : 1.0.0
2020-02-24T23:53:38.1226059Z Author       : Microsoft
2020-02-24T23:53:38.1226059Z Author       : Microsoft
2020-02-24T23:53:38.1226416Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-24T23:53:38.1226802Z ==============================================================================
2020-02-24T23:53:38.4233899Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-24T23:53:38.4302801Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69363/merge to s
2020-02-24T23:53:38.4386599Z Cleaning up task key
2020-02-24T23:53:38.4387767Z Start cleaning up orphan processes.
2020-02-24T23:53:38.4538576Z Terminate orphan process: pid (4172) (python)
2020-02-24T23:53:38.4674044Z ##[section]Finishing: Finalize Job
