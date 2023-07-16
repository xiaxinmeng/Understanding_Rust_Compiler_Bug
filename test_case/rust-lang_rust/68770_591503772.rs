plain
2020-02-26T15:30:48.9930082Z ========================== Starting Command Output ===========================
2020-02-26T15:30:48.9933077Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3cbafc2f-0a66-4143-9642-6e03193d0463.sh
2020-02-26T15:30:48.9933422Z 
2020-02-26T15:30:48.9940778Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T15:30:48.9971986Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68770/merge to s
2020-02-26T15:30:48.9976200Z Task         : Get sources
2020-02-26T15:30:48.9976568Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T15:30:48.9976861Z Version      : 1.0.0
2020-02-26T15:30:48.9977058Z Author       : Microsoft
---
2020-02-26T15:30:49.9998561Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T15:30:50.0004128Z ##[command]git config gc.auto 0
2020-02-26T15:30:50.0008986Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T15:30:50.0012890Z ##[command]git config --get-all http.proxy
2020-02-26T15:30:50.0018931Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68770/merge:refs/remotes/pull/68770/merge
---
2020-02-26T16:02:45.2353403Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2020-02-26T16:02:45.6202110Z error: unused doc comment
2020-02-26T16:02:45.6202859Z   --> src/liballoc/collections/btree/navigate.rs:34:5
2020-02-26T16:02:45.6203376Z    |
2020-02-26T16:02:45.6204081Z 34 |     /// Returns a handle to the KV right of the leaf edge, either in the leaf itself or
2020-02-26T16:02:45.6205456Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macros
2020-02-26T16:02:45.6206790Z    = note: `-D unused-doc-comments` implied by `-D warnings`
2020-02-26T16:02:45.6207689Z    = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
2020-02-26T16:02:45.6208080Z 
2020-02-26T16:02:45.6246408Z error: unused doc comment
2020-02-26T16:02:45.6246408Z error: unused doc comment
2020-02-26T16:02:45.6247057Z   --> src/liballoc/collections/btree/navigate.rs:35:5
2020-02-26T16:02:45.6247581Z    |
2020-02-26T16:02:45.6248274Z 35 |     /// in a parent. If the leaf edge is the last one in the tree, returns the root node.
2020-02-26T16:02:45.6249350Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macros
2020-02-26T16:02:45.6251008Z    = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
2020-02-26T16:02:45.6251423Z 
2020-02-26T16:02:45.6251971Z error: unused doc comment
2020-02-26T16:02:45.6252567Z   --> src/liballoc/collections/btree/navigate.rs:38:5
2020-02-26T16:02:45.6252567Z   --> src/liballoc/collections/btree/navigate.rs:38:5
2020-02-26T16:02:45.6253600Z    |
2020-02-26T16:02:45.6257116Z 38 |     /// Returns a handle to the KV left of the leaf edge, either in the leaf itself or
2020-02-26T16:02:45.6258950Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macros
2020-02-26T16:02:45.6260523Z    = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
2020-02-26T16:02:45.6260913Z 
2020-02-26T16:02:45.6278933Z error: unused doc comment
2020-02-26T16:02:45.6279593Z   --> src/liballoc/collections/btree/navigate.rs:39:5
2020-02-26T16:02:45.6279593Z   --> src/liballoc/collections/btree/navigate.rs:39:5
2020-02-26T16:02:45.6280167Z    |
2020-02-26T16:02:45.6280873Z 39 |     /// in a parent. If the leaf edge is the first one in the tree, returns the root node.
2020-02-26T16:02:45.6281972Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macros
2020-02-26T16:02:45.6283550Z    = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
2020-02-26T16:02:45.6283946Z 
2020-02-26T16:02:45.7576745Z    Compiling cfg-if v0.1.10
2020-02-26T16:02:45.8474493Z    Compiling rustc-demangle v0.1.16
---
2020-02-26T16:02:47.5062109Z   local time: Wed Feb 26 16:02:47 UTC 2020
2020-02-26T16:02:47.8588176Z   network time: Wed, 26 Feb 2020 16:02:47 GMT
2020-02-26T16:02:47.8591621Z == end clock drift check ==
2020-02-26T16:02:49.8386796Z 
2020-02-26T16:02:49.8453954Z ##[error]Bash exited with code '1'.
2020-02-26T16:02:49.8471050Z ##[section]Finishing: Run build
2020-02-26T16:02:49.8523485Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68770/merge to s
2020-02-26T16:02:49.8529481Z Task         : Get sources
2020-02-26T16:02:49.8529768Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T16:02:49.8530050Z Version      : 1.0.0
2020-02-26T16:02:49.8530239Z Author       : Microsoft
2020-02-26T16:02:49.8530239Z Author       : Microsoft
2020-02-26T16:02:49.8530531Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T16:02:49.8530886Z ==============================================================================
2020-02-26T16:02:50.1976443Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-26T16:02:50.2023297Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68770/merge to s
2020-02-26T16:02:50.2123904Z Cleaning up task key
2020-02-26T16:02:50.2125138Z Start cleaning up orphan processes.
2020-02-26T16:02:50.2293044Z Terminate orphan process: pid (4215) (python)
2020-02-26T16:02:50.2477511Z ##[section]Finishing: Finalize Job
