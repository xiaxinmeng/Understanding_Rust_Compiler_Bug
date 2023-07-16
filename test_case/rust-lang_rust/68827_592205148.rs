plain
2020-02-27T22:08:38.6739167Z ========================== Starting Command Output ===========================
2020-02-27T22:08:38.6742601Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0c0122db-32e7-4431-95c9-7d8efb756f22.sh
2020-02-27T22:08:38.6742967Z 
2020-02-27T22:08:38.6746951Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-27T22:08:38.6762051Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68827/merge to s
2020-02-27T22:08:38.6765020Z Task         : Get sources
2020-02-27T22:08:38.6765238Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T22:08:38.6765449Z Version      : 1.0.0
2020-02-27T22:08:38.6765593Z Author       : Microsoft
---
2020-02-27T22:08:39.6897962Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-27T22:08:39.6903823Z ##[command]git config gc.auto 0
2020-02-27T22:08:39.6910247Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-27T22:08:39.6919946Z ##[command]git config --get-all http.proxy
2020-02-27T22:08:39.6925089Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68827/merge:refs/remotes/pull/68827/merge
---
2020-02-27T22:13:43.2328572Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-02-27T22:13:43.2607603Z    Compiling hashbrown v0.6.2
2020-02-27T22:13:46.3032532Z     Checking cfg-if v0.1.10
2020-02-27T22:13:46.3432985Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-02-27T22:13:46.9935651Z error[E0599]: no method named `forget_node_type` found for struct `collections::btree::node::Handle<collections::btree::node::NodeRef<collections::btree::node::marker::Mut<'_>, K, V, collections::btree::node::marker::Leaf>, collections::btree::node::marker::KV>` in the current scope
2020-02-27T22:13:46.9937203Z     |
2020-02-27T22:13:46.9937203Z     |
2020-02-27T22:13:46.9937743Z 680 |                 handle: kv.forget_node_type(),
2020-02-27T22:13:46.9968520Z     |                            ^^^^^^^^^^^^^^^^ method not found in `collections::btree::node::Handle<collections::btree::node::NodeRef<collections::btree::node::marker::Mut<'_>, K, V, collections::btree::node::marker::Leaf>, collections::btree::node::marker::KV>`
2020-02-27T22:13:46.9970341Z    ::: src/liballoc/collections/btree/node.rs:782:1
2020-02-27T22:13:46.9970955Z     |
2020-02-27T22:13:46.9970955Z     |
2020-02-27T22:13:46.9971480Z 782 | pub struct Handle<Node, Type> {
2020-02-27T22:13:46.9972191Z     | ----------------------------- method `forget_node_type` not found for this
2020-02-27T22:13:46.9976340Z 
2020-02-27T22:13:47.0015778Z error[E0599]: no method named `forget_node_type` found for struct `collections::btree::node::Handle<collections::btree::node::NodeRef<collections::btree::node::marker::Mut<'_>, K, V, collections::btree::node::marker::Leaf>, collections::btree::node::marker::KV>` in the current scope
2020-02-27T22:13:47.0017429Z     |
2020-02-27T22:13:47.0017429Z     |
2020-02-27T22:13:47.0017976Z 743 |                 handle: kv.forget_node_type(),
2020-02-27T22:13:47.0019257Z     |                            ^^^^^^^^^^^^^^^^ method not found in `collections::btree::node::Handle<collections::btree::node::NodeRef<collections::btree::node::marker::Mut<'_>, K, V, collections::btree::node::marker::Leaf>, collections::btree::node::marker::KV>`
2020-02-27T22:13:47.0020637Z    ::: src/liballoc/collections/btree/node.rs:782:1
2020-02-27T22:13:47.0021065Z     |
2020-02-27T22:13:47.0021065Z     |
2020-02-27T22:13:47.0021525Z 782 | pub struct Handle<Node, Type> {
2020-02-27T22:13:47.0022235Z     | ----------------------------- method `forget_node_type` not found for this
2020-02-27T22:13:47.1497590Z     Checking rustc-demangle v0.1.16
2020-02-27T22:13:47.3916166Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-02-27T22:13:47.5201805Z     Checking backtrace v0.3.44
2020-02-27T22:13:47.5825119Z error: aborting due to 2 previous errors
---
2020-02-27T22:13:47.6851237Z   local time: Thu Feb 27 22:13:47 UTC 2020
2020-02-27T22:13:48.2231992Z   network time: Thu, 27 Feb 2020 22:13:48 GMT
2020-02-27T22:13:48.2237597Z == end clock drift check ==
2020-02-27T22:13:49.2268342Z 
2020-02-27T22:13:49.2328430Z ##[error]Bash exited with code '1'.
2020-02-27T22:13:49.2351783Z ##[section]Finishing: Run build
2020-02-27T22:13:49.2389824Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68827/merge to s
2020-02-27T22:13:49.2393985Z Task         : Get sources
2020-02-27T22:13:49.2394271Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T22:13:49.2394537Z Version      : 1.0.0
2020-02-27T22:13:49.2394738Z Author       : Microsoft
2020-02-27T22:13:49.2394738Z Author       : Microsoft
2020-02-27T22:13:49.2395026Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-27T22:13:49.2395364Z ==============================================================================
2020-02-27T22:13:49.5169374Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-27T22:13:49.5203873Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68827/merge to s
2020-02-27T22:13:49.5272396Z Cleaning up task key
2020-02-27T22:13:49.5273532Z Start cleaning up orphan processes.
2020-02-27T22:13:49.5417795Z Terminate orphan process: pid (3901) (python)
2020-02-27T22:13:49.5526230Z ##[section]Finishing: Finalize Job
