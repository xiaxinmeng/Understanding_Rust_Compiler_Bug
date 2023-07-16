plain
2019-07-24T01:35:56.9796823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T01:35:56.9983665Z ##[command]git config gc.auto 0
2019-07-24T01:35:57.0080290Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T01:35:57.0136375Z ##[command]git config --get-all http.proxy
2019-07-24T01:35:57.0353660Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-07-24T01:36:32.7715422Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T01:36:32.7715455Z 
2019-07-24T01:36:32.7715660Z   git checkout -b <new-branch-name>
2019-07-24T01:36:32.7715712Z 
2019-07-24T01:36:32.7715760Z HEAD is now at abd745e64 Merge dcd6c51cf7338ba86f1a28a856c6145653098890 into a7f28678bbf4e16893bb6a718e427504167a9494
2019-07-24T01:36:32.7871028Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T01:36:32.7874417Z ==============================================================================
2019-07-24T01:36:32.7874478Z Task         : Bash
2019-07-24T01:36:32.7874543Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T01:45:11.5519366Z     Checking lock_api v0.1.3
2019-07-24T01:45:12.0899269Z     Checking polonius-engine v0.9.0
2019-07-24T01:45:12.7438065Z    Compiling rustc_version v0.2.3
2019-07-24T01:45:14.5785687Z     Checking chalk-engine v0.9.0
2019-07-24T01:45:14.6733228Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6733638Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-24T01:45:14.6733892Z    |
2019-07-24T01:45:14.6734761Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.6735575Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.6736123Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.6736570Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.6736821Z ...    |
2019-07-24T01:45:14.6737195Z 13 |   | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-24T01:45:14.6737692Z    |   |______________________________________________________________^
2019-07-24T01:45:14.6738747Z 14 |  || & mut self ) -> Self {
2019-07-24T01:45:14.6739281Z 15 |  || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.6739931Z    |  ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6740225Z ...    |
2019-07-24T01:45:14.6740545Z 23 |   | } impl From < usize > for $ n {
2019-07-24T01:45:14.6740923Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.6741321Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.6741887Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-24T01:45:14.6742106Z    |
2019-07-24T01:45:14.6742106Z    |
2019-07-24T01:45:14.6742412Z 15 | /   index_struct! {
2019-07-24T01:45:14.6742741Z 16 | |       pub(crate) struct StackIndex {
2019-07-24T01:45:14.6743049Z 17 | |           value: usize,
2019-07-24T01:45:14.6743641Z 19 | |   }
2019-07-24T01:45:14.6743938Z    | |___- in this macro invocation
2019-07-24T01:45:14.6743998Z 
2019-07-24T01:45:14.6743998Z 
2019-07-24T01:45:14.6757702Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6760885Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-24T01:45:14.6763846Z    |
2019-07-24T01:45:14.6769432Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.6795776Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.6800828Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.6801482Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.6801921Z ...    |
2019-07-24T01:45:14.6802665Z 15 |   | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.6803191Z    |   |__________________________________________________________________^
2019-07-24T01:45:14.6804469Z 16 |  || replace_zero ( & mut self ) -> Self {
2019-07-24T01:45:14.6805302Z 17 |  || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.6806212Z    |  ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6806655Z ...    |
2019-07-24T01:45:14.6807159Z 23 |   | } impl From < usize > for $ n {
2019-07-24T01:45:14.6807685Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.6808413Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.6809416Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-24T01:45:14.6809955Z    |
2019-07-24T01:45:14.6809955Z    |
2019-07-24T01:45:14.6810624Z 15 | /   index_struct! {
2019-07-24T01:45:14.6811090Z 16 | |       pub(crate) struct StackIndex {
2019-07-24T01:45:14.6811537Z 17 | |           value: usize,
2019-07-24T01:45:14.6812444Z 19 | |   }
2019-07-24T01:45:14.6812908Z    | |___- in this macro invocation
2019-07-24T01:45:14.6813069Z 
2019-07-24T01:45:14.6813069Z 
2019-07-24T01:45:14.6813479Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6813939Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-24T01:45:14.6814315Z    |
2019-07-24T01:45:14.6836864Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.6837444Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.6837874Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.6863437Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.6872982Z ...    |
2019-07-24T01:45:14.6879369Z 17 |   | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.6889309Z    |  _|___________________________________________________________________^
2019-07-24T01:45:14.6895618Z 18 | | | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.6906371Z    | |_|___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6922920Z ...    |
2019-07-24T01:45:14.6955464Z 23 |   | } impl From < usize > for $ n {
2019-07-24T01:45:14.6955954Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.6956404Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.6956997Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-24T01:45:14.6957221Z    |
2019-07-24T01:45:14.6957221Z    |
2019-07-24T01:45:14.6957562Z 15 |  /  index_struct! {
2019-07-24T01:45:14.6957888Z 16 |  |      pub(crate) struct StackIndex {
2019-07-24T01:45:14.6958223Z 17 |  |          value: usize,
2019-07-24T01:45:14.6958538Z 18 |  |      }
2019-07-24T01:45:14.6958833Z 19 |  |  }
2019-07-24T01:45:14.6959172Z    |  |__- in this macro invocation
2019-07-24T01:45:14.6959214Z 
2019-07-24T01:45:14.6960538Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6960941Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-24T01:45:14.6961171Z    |
2019-07-24T01:45:14.6961821Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.6962469Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.6962879Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.6963246Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.6963655Z ...    |
2019-07-24T01:45:14.6964019Z 18 |   | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.6964667Z    |  _|_____________________________________________________________________________^
2019-07-24T01:45:14.6965375Z 19 | | | sub_one ( & self ) -> Self {
2019-07-24T01:45:14.6965766Z 20 | | | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.6966208Z    | |_|________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.6966474Z ...    |
2019-07-24T01:45:14.6966814Z 23 |   | } impl From < usize > for $ n {
2019-07-24T01:45:14.6967175Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.6967563Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.6968350Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-24T01:45:14.6968592Z    |
2019-07-24T01:45:14.6968592Z    |
2019-07-24T01:45:14.6969374Z 15 |  /  index_struct! {
2019-07-24T01:45:14.6969862Z 16 |  |      pub(crate) struct StackIndex {
2019-07-24T01:45:14.6970425Z 17 |  |          value: usize,
2019-07-24T01:45:14.6970721Z 18 |  |      }
2019-07-24T01:45:14.6971220Z 19 |  |  }
2019-07-24T01:45:14.6971541Z    |  |__- in this macro invocation
2019-07-24T01:45:14.7083375Z 
2019-07-24T01:45:14.7084221Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7085088Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-24T01:45:14.7085350Z    |
2019-07-24T01:45:14.7085944Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7086385Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7086756Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7087258Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7087536Z ...    |
2019-07-24T01:45:14.7087896Z 20 |   | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.7088261Z    |  _|__________________________________________________________^
2019-07-24T01:45:14.7088642Z 21 | | | & self , n : usize ) -> Option < Self > {
2019-07-24T01:45:14.7089032Z 22 | | | usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-24T01:45:14.7089657Z    | |_|______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7089988Z 23 |   | } impl From < usize > for $ n {
2019-07-24T01:45:14.7090359Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7090754Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7091316Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-24T01:45:14.7091533Z    |
2019-07-24T01:45:14.7091533Z    |
2019-07-24T01:45:14.7091812Z 15 | /   index_struct! {
2019-07-24T01:45:14.7092137Z 16 | |       pub(crate) struct StackIndex {
2019-07-24T01:45:14.7092438Z 17 | |           value: usize,
2019-07-24T01:45:14.7093015Z 19 | |   }
2019-07-24T01:45:14.7093482Z    | |___- in this macro invocation
2019-07-24T01:45:14.7093545Z 
2019-07-24T01:45:14.7093545Z 
2019-07-24T01:45:14.7093835Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7094115Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-24T01:45:14.7094349Z    |
2019-07-24T01:45:14.7095043Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7095461Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7095829Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7096301Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7096555Z ...   |
2019-07-24T01:45:14.7096938Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-24T01:45:14.7097285Z    |  |______________________________________________________________^
2019-07-24T01:45:14.7097631Z 14 | || & mut self ) -> Self {
2019-07-24T01:45:14.7098004Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.7098598Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7099287Z ...   |
2019-07-24T01:45:14.7099603Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7099976Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7100352Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7101492Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-24T01:45:14.7101715Z    |
2019-07-24T01:45:14.7101715Z    |
2019-07-24T01:45:14.7102035Z 34 | /  index_struct! {
2019-07-24T01:45:14.7102362Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-24T01:45:14.7102689Z 36 | |          value: usize,
2019-07-24T01:45:14.7103263Z 38 | |  }
2019-07-24T01:45:14.7103594Z    | |__- in this macro invocation
2019-07-24T01:45:14.7103634Z 
2019-07-24T01:45:14.7103634Z 
2019-07-24T01:45:14.7103922Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7104201Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-24T01:45:14.7104663Z    |
2019-07-24T01:45:14.7105135Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7105570Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7105916Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7106392Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7106668Z ...   |
2019-07-24T01:45:14.7107030Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.7107369Z    |  |__________________________________________________________________^
2019-07-24T01:45:14.7107734Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-24T01:45:14.7108119Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.7108585Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7108847Z ...   |
2019-07-24T01:45:14.7109189Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7109556Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7110041Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7110607Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-24T01:45:14.7110855Z    |
2019-07-24T01:45:14.7110855Z    |
2019-07-24T01:45:14.7111146Z 34 | /  index_struct! {
2019-07-24T01:45:14.7111469Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-24T01:45:14.7111799Z 36 | |          value: usize,
2019-07-24T01:45:14.7112385Z 38 | |  }
2019-07-24T01:45:14.7112697Z    | |__- in this macro invocation
2019-07-24T01:45:14.7112736Z 
2019-07-24T01:45:14.7112736Z 
2019-07-24T01:45:14.7113013Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7113313Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-24T01:45:14.7113534Z    |
2019-07-24T01:45:14.7114952Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7115444Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7115786Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7116295Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7116553Z ...   |
2019-07-24T01:45:14.7116919Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.7117282Z    |  |___________________________________________________________________^
2019-07-24T01:45:14.7117663Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.7118508Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7118782Z ...   |
2019-07-24T01:45:14.7119102Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7119492Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7119884Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7120640Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-24T01:45:14.7120878Z    |
2019-07-24T01:45:14.7120878Z    |
2019-07-24T01:45:14.7121158Z 34 | /  index_struct! {
2019-07-24T01:45:14.7121464Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-24T01:45:14.7121788Z 36 | |          value: usize,
2019-07-24T01:45:14.7122382Z 38 | |  }
2019-07-24T01:45:14.7122694Z    | |__- in this macro invocation
2019-07-24T01:45:14.7122740Z 
2019-07-24T01:45:14.7122740Z 
2019-07-24T01:45:14.7123008Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7123298Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-24T01:45:14.7123508Z    |
2019-07-24T01:45:14.7123944Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7124367Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7125146Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7125669Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7125927Z ...   |
2019-07-24T01:45:14.7126286Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.7126666Z    |  |_____________________________________________________________________________^
2019-07-24T01:45:14.7127002Z 19 | || sub_one ( & self ) -> Self {
2019-07-24T01:45:14.7127373Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.7127824Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7128082Z ...   |
2019-07-24T01:45:14.7128574Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7128925Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7129325Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7129881Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-24T01:45:14.7130100Z    |
2019-07-24T01:45:14.7130100Z    |
2019-07-24T01:45:14.7130381Z 34 | /  index_struct! {
2019-07-24T01:45:14.7130712Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-24T01:45:14.7131013Z 36 | |          value: usize,
2019-07-24T01:45:14.7131581Z 38 | |  }
2019-07-24T01:45:14.7133202Z    | |__- in this macro invocation
2019-07-24T01:45:14.7133254Z 
2019-07-24T01:45:14.7133254Z 
2019-07-24T01:45:14.7133890Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7134682Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-24T01:45:14.7134953Z    |
2019-07-24T01:45:14.7135466Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7135894Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7136233Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7136782Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7137033Z ...   |
2019-07-24T01:45:14.7137418Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.7137760Z    |  |__________________________________________________________^
2019-07-24T01:45:14.7138298Z 21 | || & self , n : usize ) -> Option < Self > {
2019-07-24T01:45:14.7138672Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-24T01:45:14.7139097Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7139439Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7139771Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7140161Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7140679Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-24T01:45:14.7140911Z    |
2019-07-24T01:45:14.7140911Z    |
2019-07-24T01:45:14.7141183Z 34 | /  index_struct! {
2019-07-24T01:45:14.7141483Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-24T01:45:14.7141793Z 36 | |          value: usize,
2019-07-24T01:45:14.7142526Z 38 | |  }
2019-07-24T01:45:14.7142833Z    | |__- in this macro invocation
2019-07-24T01:45:14.7142872Z 
2019-07-24T01:45:14.7142872Z 
2019-07-24T01:45:14.7143143Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7143436Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-24T01:45:14.7143647Z    |
2019-07-24T01:45:14.7144077Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7144474Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7145221Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7145751Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7146005Z ...   |
2019-07-24T01:45:14.7146373Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-24T01:45:14.7146737Z    |  |______________________________________________________________^
2019-07-24T01:45:14.7147783Z 14 | || & mut self ) -> Self {
2019-07-24T01:45:14.7149338Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.7149804Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7150077Z ...   |
2019-07-24T01:45:14.7150995Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7153025Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7234398Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7235335Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-24T01:45:14.7235563Z    |
2019-07-24T01:45:14.7235563Z    |
2019-07-24T01:45:14.7235858Z 81 | /  index_struct! {
2019-07-24T01:45:14.7236232Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-24T01:45:14.7236544Z 83 | |          value: usize,
2019-07-24T01:45:14.7237150Z 85 | |  }
2019-07-24T01:45:14.7237462Z    | |__- in this macro invocation
2019-07-24T01:45:14.7237506Z 
2019-07-24T01:45:14.7237506Z 
2019-07-24T01:45:14.7237963Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7238223Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-24T01:45:14.7238427Z    |
2019-07-24T01:45:14.7239330Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7239739Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7240081Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7240549Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7240787Z ...   |
2019-07-24T01:45:14.7241149Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.7241470Z    |  |__________________________________________________________________^
2019-07-24T01:45:14.7241803Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-24T01:45:14.7242174Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.7242752Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7243030Z ...   |
2019-07-24T01:45:14.7243349Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7243726Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7244101Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7245093Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-24T01:45:14.7245324Z    |
2019-07-24T01:45:14.7245324Z    |
2019-07-24T01:45:14.7245644Z 81 | /  index_struct! {
2019-07-24T01:45:14.7246052Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-24T01:45:14.7246383Z 83 | |          value: usize,
2019-07-24T01:45:14.7246972Z 85 | |  }
2019-07-24T01:45:14.7247291Z    | |__- in this macro invocation
2019-07-24T01:45:14.7247331Z 
2019-07-24T01:45:14.7247331Z 
2019-07-24T01:45:14.7247617Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7247897Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-24T01:45:14.7248400Z    |
2019-07-24T01:45:14.7248780Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7249158Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7249596Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7249946Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7250215Z ...   |
2019-07-24T01:45:14.7250925Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.7251471Z    |  |___________________________________________________________________^
2019-07-24T01:45:14.7251859Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.7252307Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7252589Z ...   |
2019-07-24T01:45:14.7252917Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7253302Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7253694Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7254565Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-24T01:45:14.7254823Z    |
2019-07-24T01:45:14.7254823Z    |
2019-07-24T01:45:14.7255144Z 81 | /  index_struct! {
2019-07-24T01:45:14.7255488Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-24T01:45:14.7255798Z 83 | |          value: usize,
2019-07-24T01:45:14.7256396Z 85 | |  }
2019-07-24T01:45:14.7256722Z    | |__- in this macro invocation
2019-07-24T01:45:14.7256763Z 
2019-07-24T01:45:14.7256763Z 
2019-07-24T01:45:14.7257047Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7257324Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-24T01:45:14.7257567Z    |
2019-07-24T01:45:14.7258028Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7258443Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7258811Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7259288Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7259568Z ...   |
2019-07-24T01:45:14.7259940Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.7260470Z    |  |_____________________________________________________________________________^
2019-07-24T01:45:14.7260818Z 19 | || sub_one ( & self ) -> Self {
2019-07-24T01:45:14.7261177Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.7261610Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7261866Z ...   |
2019-07-24T01:45:14.7262469Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7262838Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7263946Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7264560Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-24T01:45:14.7265025Z    |
2019-07-24T01:45:14.7265025Z    |
2019-07-24T01:45:14.7265321Z 81 | /  index_struct! {
2019-07-24T01:45:14.7265664Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-24T01:45:14.7265998Z 83 | |          value: usize,
2019-07-24T01:45:14.7266584Z 85 | |  }
2019-07-24T01:45:14.7266905Z    | |__- in this macro invocation
2019-07-24T01:45:14.7266946Z 
2019-07-24T01:45:14.7266946Z 
2019-07-24T01:45:14.7268022Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7268347Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-24T01:45:14.7268714Z    |
2019-07-24T01:45:14.7269118Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7269517Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7270055Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7270450Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7270710Z ...   |
2019-07-24T01:45:14.7271074Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.7271760Z    |  |__________________________________________________________^
2019-07-24T01:45:14.7272258Z 21 | || & self , n : usize ) -> Option < Self > {
2019-07-24T01:45:14.7272657Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-24T01:45:14.7273089Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7273981Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7274736Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7275537Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7276131Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-24T01:45:14.7276374Z    |
2019-07-24T01:45:14.7276374Z    |
2019-07-24T01:45:14.7276675Z 81 | /  index_struct! {
2019-07-24T01:45:14.7277019Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-24T01:45:14.7277345Z 83 | |          value: usize,
2019-07-24T01:45:14.7277923Z 85 | |  }
2019-07-24T01:45:14.7278260Z    | |__- in this macro invocation
2019-07-24T01:45:14.7278302Z 
2019-07-24T01:45:14.7278302Z 
2019-07-24T01:45:14.7278584Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7278882Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-24T01:45:14.7279224Z    |
2019-07-24T01:45:14.7279622Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7280015Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7280471Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7280836Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7281115Z ...   |
2019-07-24T01:45:14.7281482Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-24T01:45:14.7281842Z    |  |______________________________________________________________^
2019-07-24T01:45:14.7282174Z 14 | || & mut self ) -> Self {
2019-07-24T01:45:14.7282557Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.7283008Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7283275Z ...   |
2019-07-24T01:45:14.7283621Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7285025Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7285470Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7286042Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-24T01:45:14.7286290Z    |
2019-07-24T01:45:14.7286290Z    |
2019-07-24T01:45:14.7286587Z 91 | /  index_struct! {
2019-07-24T01:45:14.7287220Z 93 | |          value: usize,
2019-07-24T01:45:14.7287507Z 94 | |      }
2019-07-24T01:45:14.7287822Z 95 | |  }
2019-07-24T01:45:14.7288130Z    | |__- in this macro invocation
2019-07-24T01:45:14.7288130Z    | |__- in this macro invocation
2019-07-24T01:45:14.7288171Z 
2019-07-24T01:45:14.7288452Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7288752Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-24T01:45:14.7289103Z    |
2019-07-24T01:45:14.7289524Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7289902Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7290704Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7291097Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7291349Z ...   |
2019-07-24T01:45:14.7291706Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-24T01:45:14.7292076Z    |  |__________________________________________________________________^
2019-07-24T01:45:14.7292417Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-24T01:45:14.7292831Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.7293267Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7293553Z ...   |
2019-07-24T01:45:14.7293879Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7294241Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7294881Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7295483Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-24T01:45:14.7295705Z    |
2019-07-24T01:45:14.7295705Z    |
2019-07-24T01:45:14.7296007Z 91 | /  index_struct! {
2019-07-24T01:45:14.7296640Z 93 | |          value: usize,
2019-07-24T01:45:14.7296943Z 94 | |      }
2019-07-24T01:45:14.7297244Z 95 | |  }
2019-07-24T01:45:14.7297541Z    | |__- in this macro invocation
2019-07-24T01:45:14.7297541Z    | |__- in this macro invocation
2019-07-24T01:45:14.7297579Z 
2019-07-24T01:45:14.7297885Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7298256Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-24T01:45:14.7298518Z    |
2019-07-24T01:45:14.7298893Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7299260Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7299724Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7300095Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7300344Z ...   |
2019-07-24T01:45:14.7300737Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-24T01:45:14.7302272Z    |  |___________________________________________________________________^
2019-07-24T01:45:14.7302753Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.7303214Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7303475Z ...   |
2019-07-24T01:45:14.7303830Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7304197Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7304815Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-24T01:45:14.7305526Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-24T01:45:14.7305753Z    |
2019-07-24T01:45:14.7305753Z    |
2019-07-24T01:45:14.7306043Z 91 | /  index_struct! {
2019-07-24T01:45:14.7306689Z 93 | |          value: usize,
2019-07-24T01:45:14.7306976Z 94 | |      }
2019-07-24T01:45:14.7307285Z 95 | |  }
2019-07-24T01:45:14.7307593Z    | |__- in this macro invocation
2019-07-24T01:45:14.7307593Z    | |__- in this macro invocation
2019-07-24T01:45:14.7307634Z 
2019-07-24T01:45:14.7307937Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7308439Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-24T01:45:14.7308651Z    |
2019-07-24T01:45:14.7309161Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-24T01:45:14.7309744Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-24T01:45:14.7310103Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-24T01:45:14.7310591Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-24T01:45:14.7310841Z ...   |
2019-07-24T01:45:14.7311228Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-24T01:45:14.7311587Z    |  |_____________________________________________________________________________^
2019-07-24T01:45:14.7311920Z 19 | || sub_one ( & self ) -> Self {
2019-07-24T01:45:14.7312333Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-24T01:45:14.7312748Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-24T01:45:14.7313030Z ...   |
2019-07-24T01:45:14.7313355Z 23 |  | } impl From < usize > for $ n {
2019-07-24T01:45:14.7313737Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-24T01:45:14.7355748Z    |  |________________________________________________________________- in this expansion of `index_struct!`
