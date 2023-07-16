plain
2019-11-11T08:40:41.4737811Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T08:40:41.5008954Z ##[command]git config gc.auto 0
2019-11-11T08:40:41.5085614Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T08:40:41.5132105Z ##[command]git config --get-all http.proxy
2019-11-11T08:40:41.5270674Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66279/merge:refs/remotes/pull/66279/merge
---
2019-11-11T08:49:17.4712108Z configure: build.locked-deps    := True
2019-11-11T08:49:17.4712249Z configure: llvm.ccache          := sccache
2019-11-11T08:49:17.4712532Z configure: build.cargo-native-static := True
2019-11-11T08:49:17.4712823Z configure: dist.missing-tools   := True
2019-11-11T08:49:17.4713149Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-11-11T08:49:17.4713431Z configure: writing `config.toml` in current directory
2019-11-11T08:49:17.4713544Z configure: 
2019-11-11T08:49:17.4713831Z configure: run `python /checkout/x.py --help`
2019-11-11T08:49:17.4713971Z configure: 
---
2019-11-11T08:52:54.1183716Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-11T08:52:54.9026324Z error[E0404]: expected trait, found derive macro `HashStable`
2019-11-11T08:52:54.9027687Z    --> src/libsyntax/token.rs:267:11
2019-11-11T08:52:54.9028143Z     |
2019-11-11T08:52:54.9028618Z 267 | impl<CTX> HashStable<CTX> for TokenKind
2019-11-11T08:52:54.9029673Z help: possible better candidate is found in another module, you can import it into scope
2019-11-11T08:52:54.9030117Z     |
2019-11-11T08:52:54.9030575Z 1   | use rustc_data_structures::stable_hasher::HashStable;
2019-11-11T08:52:54.9030973Z     |
2019-11-11T08:52:54.9030973Z     |
2019-11-11T08:52:54.9031444Z 
2019-11-11T08:52:54.9076595Z error[E0412]: cannot find type `StableHasher` in this scope
2019-11-11T08:52:54.9077434Z    --> src/libsyntax/token.rs:270:55
2019-11-11T08:52:54.9077842Z     |
2019-11-11T08:52:54.9078274Z 270 |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
2019-11-11T08:52:54.9079178Z help: possible candidate is found in another module, you can import it into scope
2019-11-11T08:52:54.9079512Z     |
2019-11-11T08:52:54.9079900Z 1   | use rustc_data_structures::stable_hasher::StableHasher;
2019-11-11T08:52:54.9080413Z     |
2019-11-11T08:52:54.9080413Z     |
2019-11-11T08:52:54.9080594Z 
2019-11-11T08:52:54.9128971Z error[E0404]: expected trait, found derive macro `HashStable`
2019-11-11T08:52:54.9129485Z   --> src/libsyntax/tokenstream.rs:57:11
2019-11-11T08:52:54.9129824Z    |
2019-11-11T08:52:54.9130224Z 57 | impl<CTX> HashStable<CTX> for TokenTree
2019-11-11T08:52:54.9131053Z help: possible better candidate is found in another module, you can import it into scope
2019-11-11T08:52:54.9131436Z    |
2019-11-11T08:52:54.9132013Z 16 | use rustc_data_structures::stable_hasher::HashStable;
2019-11-11T08:52:54.9132332Z    |
2019-11-11T08:52:54.9132332Z    |
2019-11-11T08:52:54.9132511Z 
2019-11-11T08:52:54.9181183Z error[E0412]: cannot find type `StableHasher` in this scope
2019-11-11T08:52:54.9182070Z   --> src/libsyntax/tokenstream.rs:60:55
2019-11-11T08:52:54.9182395Z    |
2019-11-11T08:52:54.9182765Z 60 |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
2019-11-11T08:52:54.9183965Z help: possible candidate is found in another module, you can import it into scope
2019-11-11T08:52:54.9184331Z    |
2019-11-11T08:52:54.9184727Z 16 | use rustc_data_structures::stable_hasher::StableHasher;
2019-11-11T08:52:54.9185061Z    |
2019-11-11T08:52:54.9185061Z    |
2019-11-11T08:52:54.9185237Z 
2019-11-11T08:52:54.9232112Z error[E0404]: expected trait, found derive macro `HashStable`
2019-11-11T08:52:54.9232608Z    --> src/libsyntax/tokenstream.rs:141:11
2019-11-11T08:52:54.9232938Z     |
2019-11-11T08:52:54.9233335Z 141 | impl<CTX> HashStable<CTX> for TokenStream
2019-11-11T08:52:54.9234098Z help: possible better candidate is found in another module, you can import it into scope
2019-11-11T08:52:54.9234408Z     |
2019-11-11T08:52:54.9234760Z 16  | use rustc_data_structures::stable_hasher::HashStable;
2019-11-11T08:52:54.9235054Z     |
2019-11-11T08:52:54.9235054Z     |
2019-11-11T08:52:54.9235252Z 
2019-11-11T08:52:54.9290649Z error[E0412]: cannot find type `StableHasher` in this scope
2019-11-11T08:52:54.9291141Z    --> src/libsyntax/tokenstream.rs:144:55
2019-11-11T08:52:54.9291520Z     |
2019-11-11T08:52:54.9291962Z 144 |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
2019-11-11T08:52:54.9292836Z help: possible candidate is found in another module, you can import it into scope
2019-11-11T08:52:54.9293180Z     |
2019-11-11T08:52:54.9293578Z 16  | use rustc_data_structures::stable_hasher::StableHasher;
2019-11-11T08:52:54.9293899Z     |
---
2019-11-11T08:52:55.6065268Z   local time: Mon Nov 11 08:52:55 UTC 2019
2019-11-11T08:52:55.6758804Z   network time: Mon, 11 Nov 2019 08:52:55 GMT
2019-11-11T08:52:55.6762496Z == end clock drift check ==
2019-11-11T08:52:57.0641002Z 
2019-11-11T08:52:57.0724044Z ##[error]Bash exited with code '1'.
2019-11-11T08:52:57.0753193Z ##[section]Starting: Checkout
2019-11-11T08:52:57.0755152Z ==============================================================================
2019-11-11T08:52:57.0755207Z Task         : Get sources
2019-11-11T08:52:57.0755245Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
