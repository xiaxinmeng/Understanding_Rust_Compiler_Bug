plain
2019-07-29T16:27:47.8703081Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T16:27:47.8894330Z ##[command]git config gc.auto 0
2019-07-29T16:27:47.8972041Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T16:27:47.9032554Z ##[command]git config --get-all http.proxy
2019-07-29T16:27:47.9185028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-07-29T16:28:24.2220083Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T16:28:24.2220286Z 
2019-07-29T16:28:24.2220723Z   git checkout -b <new-branch-name>
2019-07-29T16:28:24.2221236Z 
2019-07-29T16:28:24.2221431Z HEAD is now at b0de99b5b Merge 566431f9c7bf69858dafb5413a0c7efa8a73eb2f into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-29T16:28:24.2377019Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T16:28:24.2379846Z ==============================================================================
2019-07-29T16:28:24.2379903Z Task         : Bash
2019-07-29T16:28:24.2380009Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T16:34:16.9635657Z     Checking lock_api v0.1.3
2019-07-29T16:34:17.2728717Z    Compiling rustc_version v0.2.3
2019-07-29T16:34:18.6722337Z     Checking polonius-engine v0.9.0
2019-07-29T16:34:19.4265467Z     Checking chalk-engine v0.9.0
2019-07-29T16:34:19.5118489Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5126334Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-29T16:34:19.5132818Z    |
2019-07-29T16:34:19.5138577Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5144067Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5150009Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5155609Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5212828Z ...    |
2019-07-29T16:34:19.5213275Z 13 |   | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-29T16:34:19.5213670Z    |   |______________________________________________________________^
2019-07-29T16:34:19.5214021Z 14 |  || & mut self ) -> Self {
2019-07-29T16:34:19.5214416Z 15 |  || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5214890Z    |  ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5215338Z ...    |
2019-07-29T16:34:19.5215763Z 23 |   | } impl From < usize > for $ n {
2019-07-29T16:34:19.5216149Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5216573Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5217171Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-29T16:34:19.5217423Z    |
2019-07-29T16:34:19.5217423Z    |
2019-07-29T16:34:19.5217736Z 15 | /   index_struct! {
2019-07-29T16:34:19.5218081Z 16 | |       pub(crate) struct StackIndex {
2019-07-29T16:34:19.5218731Z 17 | |           value: usize,
2019-07-29T16:34:19.5219361Z 19 | |   }
2019-07-29T16:34:19.5219672Z    | |___- in this macro invocation
2019-07-29T16:34:19.5219725Z 
2019-07-29T16:34:19.5219725Z 
2019-07-29T16:34:19.5220040Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5220339Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-29T16:34:19.5220574Z    |
2019-07-29T16:34:19.5220956Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5221343Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5221698Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5222130Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5222582Z ...    |
2019-07-29T16:34:19.5222972Z 15 |   | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5223331Z    |   |__________________________________________________________________^
2019-07-29T16:34:19.5223697Z 16 |  || replace_zero ( & mut self ) -> Self {
2019-07-29T16:34:19.5224117Z 17 |  || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5224663Z    |  ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5224999Z ...    |
2019-07-29T16:34:19.5225337Z 23 |   | } impl From < usize > for $ n {
2019-07-29T16:34:19.5225724Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5226126Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5226798Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-29T16:34:19.5227070Z    |
2019-07-29T16:34:19.5227070Z    |
2019-07-29T16:34:19.5227409Z 15 | /   index_struct! {
2019-07-29T16:34:19.5227788Z 16 | |       pub(crate) struct StackIndex {
2019-07-29T16:34:19.5228136Z 17 | |           value: usize,
2019-07-29T16:34:19.5229171Z 19 | |   }
2019-07-29T16:34:19.5229497Z    | |___- in this macro invocation
2019-07-29T16:34:19.5229537Z 
2019-07-29T16:34:19.5229537Z 
2019-07-29T16:34:19.5229851Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5230140Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-29T16:34:19.5230373Z    |
2019-07-29T16:34:19.5230753Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5231139Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5231510Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5232060Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5232332Z ...    |
2019-07-29T16:34:19.5232726Z 17 |   | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5233106Z    |  _|___________________________________________________________________^
2019-07-29T16:34:19.5233529Z 18 | | | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5234070Z    | |_|___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5234398Z ...    |
2019-07-29T16:34:19.5234738Z 23 |   | } impl From < usize > for $ n {
2019-07-29T16:34:19.5235115Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5235541Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5236198Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-29T16:34:19.5236458Z    |
2019-07-29T16:34:19.5236458Z    |
2019-07-29T16:34:19.5236806Z 15 |  /  index_struct! {
2019-07-29T16:34:19.5237198Z 16 |  |      pub(crate) struct StackIndex {
2019-07-29T16:34:19.5237553Z 17 |  |          value: usize,
2019-07-29T16:34:19.5237893Z 18 |  |      }
2019-07-29T16:34:19.5238471Z 19 |  |  }
2019-07-29T16:34:19.5238850Z    |  |__- in this macro invocation
2019-07-29T16:34:19.5238903Z 
2019-07-29T16:34:19.5239226Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5239515Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-29T16:34:19.5239748Z    |
2019-07-29T16:34:19.5240132Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5240516Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5240884Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5241451Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5241722Z ...    |
2019-07-29T16:34:19.5242115Z 18 |   | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5242501Z    |  _|_____________________________________________________________________________^
2019-07-29T16:34:19.5242873Z 19 | | | sub_one ( & self ) -> Self {
2019-07-29T16:34:19.5243271Z 20 | | | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:19.5243786Z    | |_|________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5244120Z ...    |
2019-07-29T16:34:19.5244458Z 23 |   | } impl From < usize > for $ n {
2019-07-29T16:34:19.5244853Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5245258Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5245921Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-29T16:34:19.5246184Z    |
2019-07-29T16:34:19.5246184Z    |
2019-07-29T16:34:19.5246540Z 15 |  /  index_struct! {
2019-07-29T16:34:19.5246906Z 16 |  |      pub(crate) struct StackIndex {
2019-07-29T16:34:19.5342729Z 17 |  |          value: usize,
2019-07-29T16:34:19.5345337Z 18 |  |      }
2019-07-29T16:34:19.5345761Z 19 |  |  }
2019-07-29T16:34:19.5346119Z    |  |__- in this macro invocation
2019-07-29T16:34:19.5346163Z 
2019-07-29T16:34:19.5346462Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5346754Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-29T16:34:19.5347010Z    |
2019-07-29T16:34:19.5347378Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5347781Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5348442Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5348847Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5349140Z ...    |
2019-07-29T16:34:19.5349512Z 20 |   | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:19.5349899Z    |  _|__________________________________________________________^
2019-07-29T16:34:19.5350270Z 21 | | | & self , n : usize ) -> Option < Self > {
2019-07-29T16:34:19.5350781Z 22 | | | usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-29T16:34:19.5351302Z    | |_|______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5351654Z 23 |   | } impl From < usize > for $ n {
2019-07-29T16:34:19.5352098Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5352501Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5353115Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-29T16:34:19.5353353Z    |
2019-07-29T16:34:19.5353353Z    |
2019-07-29T16:34:19.5353673Z 15 | /   index_struct! {
2019-07-29T16:34:19.5354004Z 16 | |       pub(crate) struct StackIndex {
2019-07-29T16:34:19.5354353Z 17 | |           value: usize,
2019-07-29T16:34:19.5359503Z 19 | |   }
2019-07-29T16:34:19.5359896Z    | |___- in this macro invocation
2019-07-29T16:34:19.5359980Z 
2019-07-29T16:34:19.5359980Z 
2019-07-29T16:34:19.5360540Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5360868Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-29T16:34:19.5361154Z    |
2019-07-29T16:34:19.5361564Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5361991Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5362660Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5363088Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5363403Z ...   |
2019-07-29T16:34:19.5363820Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-29T16:34:19.5364213Z    |  |______________________________________________________________^
2019-07-29T16:34:19.5364611Z 14 | || & mut self ) -> Self {
2019-07-29T16:34:19.5365143Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5365687Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5365992Z ...   |
2019-07-29T16:34:19.5366360Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5366787Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5367240Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5367906Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-29T16:34:19.5368170Z    |
2019-07-29T16:34:19.5368170Z    |
2019-07-29T16:34:19.5448504Z 34 | /  index_struct! {
2019-07-29T16:34:19.5449002Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-29T16:34:19.5449369Z 36 | |          value: usize,
2019-07-29T16:34:19.5450071Z 38 | |  }
2019-07-29T16:34:19.5450426Z    | |__- in this macro invocation
2019-07-29T16:34:19.5450489Z 
2019-07-29T16:34:19.5450489Z 
2019-07-29T16:34:19.5450825Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5451148Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-29T16:34:19.5451426Z    |
2019-07-29T16:34:19.5451836Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5452265Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5452948Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5453377Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5453689Z ...   |
2019-07-29T16:34:19.5454098Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5454495Z    |  |__________________________________________________________________^
2019-07-29T16:34:19.5454910Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-29T16:34:19.5455464Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5456015Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5456320Z ...   |
2019-07-29T16:34:19.5456688Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5457116Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5457583Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5458545Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-29T16:34:19.5458823Z    |
2019-07-29T16:34:19.5458823Z    |
2019-07-29T16:34:19.5459161Z 34 | /  index_struct! {
2019-07-29T16:34:19.5459555Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-29T16:34:19.5459909Z 36 | |          value: usize,
2019-07-29T16:34:19.5460612Z 38 | |  }
2019-07-29T16:34:19.5460959Z    | |__- in this macro invocation
2019-07-29T16:34:19.5461020Z 
2019-07-29T16:34:19.5461020Z 
2019-07-29T16:34:19.5461344Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5461665Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-29T16:34:19.5461937Z    |
2019-07-29T16:34:19.5462344Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5462775Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5463365Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5463794Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5464091Z ...   |
2019-07-29T16:34:19.5464526Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5464925Z    |  |___________________________________________________________________^
2019-07-29T16:34:19.5465479Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5466022Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5466323Z ...   |
2019-07-29T16:34:19.5466710Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5467123Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5467583Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5468483Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-29T16:34:19.5468821Z    |
2019-07-29T16:34:19.5468821Z    |
2019-07-29T16:34:19.5469161Z 34 | /  index_struct! {
2019-07-29T16:34:19.5469528Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-29T16:34:19.5469903Z 36 | |          value: usize,
2019-07-29T16:34:19.5470573Z 38 | |  }
2019-07-29T16:34:19.5470947Z    | |__- in this macro invocation
2019-07-29T16:34:19.5470993Z 
2019-07-29T16:34:19.5470993Z 
2019-07-29T16:34:19.5471315Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5471655Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-29T16:34:19.5471912Z    |
2019-07-29T16:34:19.5472318Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5472764Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5473337Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5473790Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5474092Z ...   |
2019-07-29T16:34:19.5474510Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5474935Z    |  |_____________________________________________________________________________^
2019-07-29T16:34:19.5475327Z 19 | || sub_one ( & self ) -> Self {
2019-07-29T16:34:19.5475872Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:19.5476392Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5476691Z ...   |
2019-07-29T16:34:19.5477076Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5477488Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5477951Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5479019Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-29T16:34:19.5479302Z    |
2019-07-29T16:34:19.5479302Z    |
2019-07-29T16:34:19.5479643Z 34 | /  index_struct! {
2019-07-29T16:34:19.5480011Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-29T16:34:19.5480383Z 36 | |          value: usize,
2019-07-29T16:34:19.5481060Z 38 | |  }
2019-07-29T16:34:19.5481433Z    | |__- in this macro invocation
2019-07-29T16:34:19.5481476Z 
2019-07-29T16:34:19.5481476Z 
2019-07-29T16:34:19.5481802Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5482139Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-29T16:34:19.5482400Z    |
2019-07-29T16:34:19.5482806Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5483251Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5483813Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5484244Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5484558Z ...   |
2019-07-29T16:34:19.5484968Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:19.5485372Z    |  |__________________________________________________________^
2019-07-29T16:34:19.5485778Z 21 | || & self , n : usize ) -> Option < Self > {
2019-07-29T16:34:19.5486312Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-29T16:34:19.5486905Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5487546Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5488004Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5488755Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5489505Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-29T16:34:19.5489770Z    |
2019-07-29T16:34:19.5489770Z    |
2019-07-29T16:34:19.5490125Z 34 | /  index_struct! {
2019-07-29T16:34:19.5490496Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-29T16:34:19.5490850Z 36 | |          value: usize,
2019-07-29T16:34:19.5491538Z 38 | |  }
2019-07-29T16:34:19.5491894Z    | |__- in this macro invocation
2019-07-29T16:34:19.5491954Z 
2019-07-29T16:34:19.5491954Z 
2019-07-29T16:34:19.5492285Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5492607Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-29T16:34:19.5492880Z    |
2019-07-29T16:34:19.5493288Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5493715Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5494458Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5494894Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5495191Z ...   |
2019-07-29T16:34:19.5495630Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-29T16:34:19.5496024Z    |  |______________________________________________________________^
2019-07-29T16:34:19.5496424Z 14 | || & mut self ) -> Self {
2019-07-29T16:34:19.5496962Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5497487Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5497808Z ...   |
2019-07-29T16:34:19.5498177Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5498996Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5499474Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5500129Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-29T16:34:19.5500391Z    |
2019-07-29T16:34:19.5500391Z    |
2019-07-29T16:34:19.5500731Z 81 | /  index_struct! {
2019-07-29T16:34:19.5501134Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-29T16:34:19.5501493Z 83 | |          value: usize,
2019-07-29T16:34:19.5502199Z 85 | |  }
2019-07-29T16:34:19.5502549Z    | |__- in this macro invocation
2019-07-29T16:34:19.5502594Z 
2019-07-29T16:34:19.5502594Z 
2019-07-29T16:34:19.5502939Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5503264Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-29T16:34:19.5503523Z    |
2019-07-29T16:34:19.5503945Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5504380Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5504958Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5505388Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5505685Z ...   |
2019-07-29T16:34:19.5506108Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5506508Z    |  |__________________________________________________________________^
2019-07-29T16:34:19.5506935Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-29T16:34:19.5507461Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5507984Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5508589Z ...   |
2019-07-29T16:34:19.5508963Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5509374Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5509866Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5510512Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-29T16:34:19.5510776Z    |
2019-07-29T16:34:19.5510776Z    |
2019-07-29T16:34:19.5511114Z 81 | /  index_struct! {
2019-07-29T16:34:19.5511517Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-29T16:34:19.5511875Z 83 | |          value: usize,
2019-07-29T16:34:19.5512574Z 85 | |  }
2019-07-29T16:34:19.5512924Z    | |__- in this macro invocation
2019-07-29T16:34:19.5512967Z 
2019-07-29T16:34:19.5512967Z 
2019-07-29T16:34:19.5513304Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5513627Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-29T16:34:19.5513886Z    |
2019-07-29T16:34:19.5514307Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5514883Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5515288Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5515737Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5516033Z ...   |
2019-07-29T16:34:19.5516465Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5516866Z    |  |___________________________________________________________________^
2019-07-29T16:34:19.5517405Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5517963Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5518623Z ...   |
2019-07-29T16:34:19.5519057Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5519471Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5519942Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5520591Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-29T16:34:19.5520854Z    |
2019-07-29T16:34:19.5520854Z    |
2019-07-29T16:34:19.5521211Z 81 | /  index_struct! {
2019-07-29T16:34:19.5521597Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-29T16:34:19.5521982Z 83 | |          value: usize,
2019-07-29T16:34:19.5522666Z 85 | |  }
2019-07-29T16:34:19.5523030Z    | |__- in this macro invocation
2019-07-29T16:34:19.5523074Z 
2019-07-29T16:34:19.5523074Z 
2019-07-29T16:34:19.5523398Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5523734Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-29T16:34:19.5523993Z    |
2019-07-29T16:34:19.5524397Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5524839Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5525423Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5525853Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5526167Z ...   |
2019-07-29T16:34:19.5526583Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5527035Z    |  |_____________________________________________________________________________^
2019-07-29T16:34:19.5647978Z 19 | || sub_one ( & self ) -> Self {
2019-07-29T16:34:19.5649042Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:19.5649649Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5649962Z ...   |
2019-07-29T16:34:19.5650352Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5650770Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5651243Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5651905Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-29T16:34:19.5652167Z    |
2019-07-29T16:34:19.5652167Z    |
2019-07-29T16:34:19.5652523Z 81 | /  index_struct! {
2019-07-29T16:34:19.5652908Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-29T16:34:19.5653264Z 83 | |          value: usize,
2019-07-29T16:34:19.5653964Z 85 | |  }
2019-07-29T16:34:19.5654329Z    | |__- in this macro invocation
2019-07-29T16:34:19.5654375Z 
2019-07-29T16:34:19.5654375Z 
2019-07-29T16:34:19.5654701Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5655024Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-29T16:34:19.5655300Z    |
2019-07-29T16:34:19.5655706Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5656320Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5656731Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5657160Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5657476Z ...   |
2019-07-29T16:34:19.5657884Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:19.5658272Z    |  |__________________________________________________________^
2019-07-29T16:34:19.5658706Z 21 | || & self , n : usize ) -> Option < Self > {
2019-07-29T16:34:19.5659236Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-29T16:34:19.5659795Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5660183Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5660609Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5661079Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5661727Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-29T16:34:19.5661987Z    |
2019-07-29T16:34:19.5661987Z    |
2019-07-29T16:34:19.5662620Z 81 | /  index_struct! {
2019-07-29T16:34:19.5663038Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-29T16:34:19.5663394Z 83 | |          value: usize,
2019-07-29T16:34:19.5664098Z 85 | |  }
2019-07-29T16:34:19.5664445Z    | |__- in this macro invocation
2019-07-29T16:34:19.5664491Z 
2019-07-29T16:34:19.5664491Z 
2019-07-29T16:34:19.5664835Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5665159Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-29T16:34:19.5665417Z    |
2019-07-29T16:34:19.5665841Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5666431Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5666857Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5667285Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5667581Z ...   |
2019-07-29T16:34:19.5668224Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-29T16:34:19.5668684Z    |  |______________________________________________________________^
2019-07-29T16:34:19.5669085Z 14 | || & mut self ) -> Self {
2019-07-29T16:34:19.5669645Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5670174Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5670497Z ...   |
2019-07-29T16:34:19.5670864Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5671276Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5671762Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5672411Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-29T16:34:19.5672675Z    |
2019-07-29T16:34:19.5672675Z    |
2019-07-29T16:34:19.5673013Z 91 | /  index_struct! {
2019-07-29T16:34:19.5673734Z 93 | |          value: usize,
2019-07-29T16:34:19.5674078Z 94 | |      }
2019-07-29T16:34:19.5674431Z 95 | |  }
2019-07-29T16:34:19.5674783Z    | |__- in this macro invocation
2019-07-29T16:34:19.5674783Z    | |__- in this macro invocation
2019-07-29T16:34:19.5674829Z 
2019-07-29T16:34:19.5675173Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5675499Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-29T16:34:19.5675755Z    |
2019-07-29T16:34:19.5676174Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5676752Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5677160Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5677606Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5677902Z ...   |
2019-07-29T16:34:19.5678556Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-29T16:34:19.5679001Z    |  |__________________________________________________________________^
2019-07-29T16:34:19.5679521Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-29T16:34:19.5680031Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5680522Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5680905Z ...   |
2019-07-29T16:34:19.5681282Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5681712Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5682184Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5682815Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-29T16:34:19.5683095Z    |
2019-07-29T16:34:19.5683095Z    |
2019-07-29T16:34:19.5683433Z 91 | /  index_struct! {
2019-07-29T16:34:19.5684154Z 93 | |          value: usize,
2019-07-29T16:34:19.5684507Z 94 | |      }
2019-07-29T16:34:19.5684852Z 95 | |  }
2019-07-29T16:34:19.5685202Z    | |__- in this macro invocation
2019-07-29T16:34:19.5685202Z    | |__- in this macro invocation
2019-07-29T16:34:19.5685246Z 
2019-07-29T16:34:19.5685567Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5685905Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-29T16:34:19.5686162Z    |
2019-07-29T16:34:19.5686568Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:19.5687323Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:19.5687761Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:19.5688203Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:19.5688894Z ...   |
2019-07-29T16:34:19.5689312Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-29T16:34:19.5689732Z    |  |___________________________________________________________________^
2019-07-29T16:34:19.5690315Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:19.5690877Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:19.5691180Z ...   |
2019-07-29T16:34:19.5691547Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:19.5691975Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:19.5692442Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-29T16:34:19.5693091Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-29T16:34:19.5693352Z    |
2019-07-29T16:34:19.5693352Z    |
2019-07-29T16:34:19.5693703Z 91 | /  index_struct! {
2019-07-29T16:34:19.5694407Z 93 | |          value: usize,
2019-07-29T16:34:19.5694768Z 94 | |      }
2019-07-29T16:34:19.5695101Z 95 | |  }
2019-07-29T16:34:19.5695448Z    | |__- in this macro invocation
2019-07-29T16:34:19.5695448Z    | |__- in this macro invocation
2019-07-29T16:34:19.5695509Z 
2019-07-29T16:34:20.2849305Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-29T16:34:20.2861153Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-29T16:34:20.2861511Z    |
2019-07-29T16:34:20.2861935Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-29T16:34:20.2862391Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-29T16:34:20.2863236Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-29T16:34:20.2863673Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-29T16:34:20.2863995Z ...   |
2019-07-29T16:34:20.2864420Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-29T16:34:20.2864852Z    |  |_____________________________________________________________________________^
2019-07-29T16:34:20.2865255Z 19 | || sub_one ( & self ) -> Self {
2019-07-29T16:34:20.2865811Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-29T16:34:20.2866360Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-29T16:34:20.2866670Z ...   |
2019-07-29T16:34:20.2867060Z 23 |  | } impl From < usize > for $ n {
2019-07-29T16:34:20.2867482Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-29T16:34:20.2867972Z    |  |________________________________________________________________- in this expansion of `index_struct!`
