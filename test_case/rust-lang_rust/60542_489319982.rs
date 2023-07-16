plain
travis_time:end:19fdd00e:start=1556970234011461764,finish=1556970319010694282,duration=84999232518
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:26]    Compiling polonius-engine v0.7.0
[00:05:27]    Compiling chalk-engine v0.9.0
[00:05:27]    Compiling serialize v0.0.0 (/checkout/src/libserialize)
[00:05:28]    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[00:05:29] error[E0046]: not all trait items implemented, missing: `sub_usize`
[00:05:29]   --> <::chalk_macros::index::index_struct macros>:10:69
[00:05:29]    |
[00:05:29] 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
[00:05:29] 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
[00:05:29] 3  |  | struct $ n { $ vf value : usize , } impl $ n {
[00:05:29] 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
[00:05:29] ...   |
[00:05:29] 10 |  | write ! ( fmt , "{}({})" , stringify ! ( $ n ) , self . value ) } } impl ::
[00:05:29]    |  |_____________________________________________________________________^
[00:05:29] 11 | || std :: iter :: Step for $ n {
[00:05:29] 12 | || fn steps_between ( start : & Self , end : & Self ) -> Option < usize > {
[00:05:29] 13 | || usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
[00:05:29] ...  ||
[00:05:29] 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
[00:05:29] 23 | || } impl From < usize > for $ n {
[00:05:29]    | ||_^ missing `sub_usize` in implementation
[00:05:29] 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
[00:05:29]    |  |________________________________________________________________- in this expansion of `index_struct!`
[00:05:29]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
[00:05:29]    |
[00:05:29]    |
[00:05:29] 81 | /  index_struct! {
[00:05:29] 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
[00:05:29] 84 | |      }
[00:05:29] 85 | |  }
[00:05:29]    | |__- in this macro invocation
[00:05:29]    |
[00:05:29]    |
[00:05:29]    = note: `sub_usize` from trait: `fn(&Self, usize) -> std::option::Option<Self>`
[00:05:29] 
[00:05:29] error[E0046]: not all trait items implemented, missing: `sub_usize`
[00:05:29]   --> <::chalk_macros::index::index_struct macros>:10:69
[00:05:29]    |
[00:05:29] 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
[00:05:29] 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
[00:05:29] 3  |  | struct $ n { $ vf value : usize , } impl $ n {
[00:05:29] 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
[00:05:29] ...   |
[00:05:29] 10 |  | write ! ( fmt , "{}({})" , stringify ! ( $ n ) , self . value ) } } impl ::
[00:05:29]    |  |_____________________________________________________________________^
[00:05:29] 11 | || std :: iter :: Step for $ n {
[00:05:29] 12 | || fn steps_between ( start : & Self , end : & Self ) -> Option < usize > {
[00:05:29] 13 | || usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
[00:05:29] ...  ||
[00:05:29] 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
[00:05:29] 23 | || } impl From < usize > for $ n {
[00:05:29]    | ||_^ missing `sub_usize` in implementation
[00:05:29] 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
[00:05:29]    |  |________________________________________________________________- in this expansion of `index_struct!`
[00:05:29]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
[00:05:29]    |
[00:05:29]    |
[00:05:29] 91 | /  index_struct! {
[00:05:29] 93 | |          value: usize,
[00:05:29] 94 | |      }
[00:05:29] 95 | |  }
[00:05:29]    | |__- in this macro invocation
[00:05:29]    | |__- in this macro invocation
[00:05:29]    |
[00:05:29]    = note: `sub_usize` from trait: `fn(&Self, usize) -> std::option::Option<Self>`
[00:05:29] 
[00:05:29] error[E0046]: not all trait items implemented, missing: `sub_usize`
[00:05:29]   --> <::chalk_macros::index::index_struct macros>:10:69
[00:05:29]    |
[00:05:29] 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
[00:05:29] 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
[00:05:29] 3  |   | struct $ n { $ vf value : usize , } impl $ n {
[00:05:29] 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
[00:05:29] ...    |
[00:05:29] 10 |   | write ! ( fmt , "{}({})" , stringify ! ( $ n ) , self . value ) } } impl ::
[00:05:29]    |   |_____________________________________________________________________^
[00:05:29] 11 |  || std :: iter :: Step for $ n {
[00:05:29] 12 |  || fn steps_between ( start : & Self , end : & Self ) -> Option < usize > {
[00:05:29] 13 |  || usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
[00:05:29] ...   ||
[00:05:29] 22 |  || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
[00:05:29] 23 |  || } impl From < usize > for $ n {
[00:05:29]    |  ||_^ missing `sub_usize` in implementation
[00:05:29] 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
[00:05:29]    |   |________________________________________________________________- in this expansion of `index_struct!`
[00:05:29]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
[00:05:29]    |
[00:05:29]    |
[00:05:29] 15 | /   index_struct! {
[00:05:29] 16 | |       pub(crate) struct StackIndex {
[00:05:29] 18 | |       }
[00:05:29] 19 | |   }
[00:05:29]    | |___- in this macro invocation
[00:05:29]    |
[00:05:29]    |
[00:05:29]    = note: `sub_usize` from trait: `fn(&Self, usize) -> std::option::Option<Self>`
[00:05:29] 
[00:05:29] error[E0046]: not all trait items implemented, missing: `sub_usize`
[00:05:29]   --> <::chalk_macros::index::index_struct macros>:10:69
[00:05:29]    |
[00:05:29] 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
[00:05:29] 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
[00:05:29] 3  |  | struct $ n { $ vf value : usize , } impl $ n {
[00:05:29] 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
[00:05:29] ...   |
[00:05:29] 10 |  | write ! ( fmt , "{}({})" , stringify ! ( $ n ) , self . value ) } } impl ::
[00:05:29]    |  |_____________________________________________________________________^
[00:05:29] 11 | || std :: iter :: Step for $ n {
[00:05:29] 12 | || fn steps_between ( start : & Self , end : & Self ) -> Option < usize > {
[00:05:29] 13 | || usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
[00:05:29] ...  ||
[00:05:29] 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
[00:05:29] 23 | || } impl From < usize > for $ n {
[00:05:29]    | ||_^ missing `sub_usize` in implementation
[00:05:29] 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
[00:05:29]    |  |________________________________________________________________- in this expansion of `index_struct!`
[00:05:29]   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
[00:05:29]    |
[00:05:29]    |
[00:05:29] 34 | /  index_struct! {
[00:05:29] 35 | |      pub(crate) struct AnswerIndex {
[00:05:29] 37 | |      }
[00:05:29] 38 | |  }
[00:05:29]    | |__- in this macro invocation
[00:05:29]    |
---
travis_time:end:06bdbbf4:start=1556970665511600523,finish=1556970665516285813,duration=4685290
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c557c5d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00270a5c
travis_time:start:00270a5c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-
