plain
2019-10-01T00:04:38.2485887Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T00:04:39.2147084Z ##[command]git config gc.auto 0
2019-10-01T00:04:39.2151730Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T00:04:39.2154990Z ##[command]git config --get-all http.proxy
2019-10-01T00:04:39.2158593Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64935/merge:refs/remotes/pull/64935/merge
---
2019-10-01T00:08:12.9929366Z Looks like docker image is the same as before, not uploading
2019-10-01T00:08:15.1201655Z [CI_JOB_NAME=mingw-check]
2019-10-01T00:08:15.3139811Z [CI_JOB_NAME=mingw-check]
2019-10-01T00:08:15.3171146Z == clock drift check ==
2019-10-01T00:08:15.3180294Z   local time: Tue Oct  1 00:08:15 UTC 2019
2019-10-01T00:08:15.4029471Z   network time: Tue, 01 Oct 2019 00:08:15 GMT
2019-10-01T00:08:15.4068769Z Starting sccache server...
2019-10-01T00:08:15.4971199Z configure: processing command line
2019-10-01T00:08:15.4971272Z configure: 
2019-10-01T00:08:15.4971973Z configure: rust.parallel-compiler := True
---
2019-10-01T00:12:51.1224212Z     Checking arena v0.0.0 (/checkout/src/libarena)
2019-10-01T00:12:56.1803825Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-01T00:13:04.9955773Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-01T00:13:06.5606701Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-01T00:13:07.5841966Z error[E0507]: cannot move out of `next.label` which is behind a shared reference
2019-10-01T00:13:07.5844517Z     |
2019-10-01T00:13:07.5844517Z     |
2019-10-01T00:13:07.5845164Z 763 |                     let l = next.label.map_or(0, |label| label.len() + 2);
2019-10-01T00:13:07.5846904Z     |                             |
2019-10-01T00:13:07.5846904Z     |                             |
2019-10-01T00:13:07.5848348Z     |                             move occurs because `next.label` has type `std::option::Option<std::string::String>`, which does not implement the `Copy` trait
2019-10-01T00:13:07.5849081Z     |                             help: consider borrowing the `Option`'s content: `next.label.as_ref()`
2019-10-01T00:13:07.6598161Z error[E0382]: borrow of moved value: `file_vec`
2019-10-01T00:13:07.6600154Z     --> src/librustc_errors/emitter.rs:1678:13
2019-10-01T00:13:07.6600407Z      |
2019-10-01T00:13:07.6600407Z      |
2019-10-01T00:13:07.6600807Z 1653 |         fn add_annotation_to_file(file_vec: &mut Vec<FileWithAnnotatedLines>,
2019-10-01T00:13:07.6601254Z      |                                   -------- move occurs because `file_vec` has type `&mut std::vec::Vec<emitter::FileWithAnnotatedLines>`, which does not implement the `Copy` trait
2019-10-01T00:13:07.6601490Z ...
2019-10-01T00:13:07.6601758Z 1658 |             for slot in file_vec {
2019-10-01T00:13:07.6602323Z      |                         |
2019-10-01T00:13:07.6603031Z      |                         value moved here
2019-10-01T00:13:07.6603031Z      |                         value moved here
2019-10-01T00:13:07.6603455Z      |                         help: consider borrowing to avoid moving into the for loop: `&file_vec`
2019-10-01T00:13:07.6603703Z ...
2019-10-01T00:13:07.6603994Z 1678 |             file_vec.push(FileWithAnnotatedLines {
2019-10-01T00:13:07.6604330Z      |             ^^^^^^^^ value borrowed here after move
2019-10-01T00:13:07.7828070Z error: aborting due to 2 previous errors
2019-10-01T00:13:07.7828469Z 
2019-10-01T00:13:07.7828856Z Some errors have detailed explanations: E0382, E0507.
2019-10-01T00:13:07.7829111Z For more information about an error, try `rustc --explain E0382`.
---
2019-10-01T00:13:09.6176998Z expected success, got: exit code: 101
2019-10-01T00:13:09.6193281Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-10-01T00:13:09.6193404Z Build completed unsuccessfully in 0:04:54
2019-10-01T00:13:09.6239833Z == clock drift check ==
2019-10-01T00:13:09.6257835Z   local time: Tue Oct  1 00:13:09 UTC 2019
2019-10-01T00:13:09.7825222Z   network time: Tue, 01 Oct 2019 00:13:09 GMT
2019-10-01T00:13:09.7830350Z == end clock drift check ==
2019-10-01T00:13:11.3806392Z ##[error]Bash exited with code '1'.
2019-10-01T00:13:11.3844741Z ##[section]Starting: Checkout
2019-10-01T00:13:11.3846571Z ==============================================================================
2019-10-01T00:13:11.3846629Z Task         : Get sources
2019-10-01T00:13:11.3846691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
