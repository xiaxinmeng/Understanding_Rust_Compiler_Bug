plain
2019-07-23T00:48:11.3263958Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T00:48:11.3460376Z ##[command]git config gc.auto 0
2019-07-23T00:48:11.3530955Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T00:48:11.3595008Z ##[command]git config --get-all http.proxy
2019-07-23T00:48:11.3729327Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-07-23T00:48:46.7767084Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T00:48:46.7767447Z 
2019-07-23T00:48:46.7768548Z   git checkout -b <new-branch-name>
2019-07-23T00:48:46.7768901Z 
2019-07-23T00:48:46.7769246Z HEAD is now at 3378debe5 Merge a3dc7a74d6c7adff8db249836d1f318a24de2de7 into e649e903440bfd919bfc9db848c28df6d795a116
2019-07-23T00:48:46.7922513Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T00:48:46.7925179Z ==============================================================================
2019-07-23T00:48:46.7925700Z Task         : Bash
2019-07-23T00:48:46.7925750Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T00:54:41.0030717Z     Checking lock_api v0.1.3
2019-07-23T00:54:41.3517875Z    Compiling rustc_version v0.2.3
2019-07-23T00:54:43.2718731Z     Checking polonius-engine v0.9.0
2019-07-23T00:54:43.7102522Z     Checking chalk-engine v0.9.0
2019-07-23T00:54:43.8071744Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8072228Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-23T00:54:43.8072458Z    |
2019-07-23T00:54:43.8072788Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:43.8073120Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:43.8073442Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:43.8073759Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:43.8074012Z ...    |
2019-07-23T00:54:43.8074334Z 13 |   | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-23T00:54:43.8074649Z    |   |______________________________________________________________^
2019-07-23T00:54:43.8074944Z 14 |  || & mut self ) -> Self {
2019-07-23T00:54:43.8075283Z 15 |  || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:43.8075681Z    |  ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8075919Z ...    |
2019-07-23T00:54:43.8076226Z 23 |   | } impl From < usize > for $ n {
2019-07-23T00:54:43.8076545Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:43.8076904Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:43.8077382Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-23T00:54:43.8077603Z    |
2019-07-23T00:54:43.8077603Z    |
2019-07-23T00:54:43.8077857Z 15 | /   index_struct! {
2019-07-23T00:54:43.8079324Z 16 | |       pub(crate) struct StackIndex {
2019-07-23T00:54:43.8080032Z 17 | |           value: usize,
2019-07-23T00:54:43.8080693Z 19 | |   }
2019-07-23T00:54:43.8081022Z    | |___- in this macro invocation
2019-07-23T00:54:43.8081199Z 
2019-07-23T00:54:43.8081199Z 
2019-07-23T00:54:43.8081539Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8081868Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-23T00:54:43.8082282Z    |
2019-07-23T00:54:43.8082642Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:43.8083490Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:43.8083860Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:43.8084271Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:43.8084560Z ...    |
2019-07-23T00:54:43.8084941Z 15 |   | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:43.8085332Z    |   |__________________________________________________________________^
2019-07-23T00:54:43.8085707Z 16 |  || replace_zero ( & mut self ) -> Self {
2019-07-23T00:54:43.8086145Z 17 |  || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:43.8086849Z    |  ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8087159Z ...    |
2019-07-23T00:54:43.8087685Z 23 |   | } impl From < usize > for $ n {
2019-07-23T00:54:43.8088748Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:43.8089213Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:43.8089839Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-23T00:54:43.8090085Z    |
2019-07-23T00:54:43.8090085Z    |
2019-07-23T00:54:43.8090527Z 15 | /   index_struct! {
2019-07-23T00:54:43.8090945Z 16 | |       pub(crate) struct StackIndex {
2019-07-23T00:54:43.8091276Z 17 | |           value: usize,
2019-07-23T00:54:43.8095687Z 19 | |   }
2019-07-23T00:54:43.8096699Z    | |___- in this macro invocation
2019-07-23T00:54:43.8096744Z 
2019-07-23T00:54:43.8096744Z 
2019-07-23T00:54:43.8097060Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8100492Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-23T00:54:43.8101056Z    |
2019-07-23T00:54:43.8101459Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:43.8101854Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:43.8102389Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:43.8102806Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:43.8103070Z ...    |
2019-07-23T00:54:43.8103466Z 17 |   | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:43.8104049Z    |  _|___________________________________________________________________^
2019-07-23T00:54:43.8104466Z 18 | | | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:43.8104957Z    | |_|___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8105242Z ...    |
2019-07-23T00:54:43.8105620Z 23 |   | } impl From < usize > for $ n {
2019-07-23T00:54:43.8106008Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:43.8106446Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:43.8107275Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-23T00:54:43.8107515Z    |
2019-07-23T00:54:43.8107515Z    |
2019-07-23T00:54:43.8107825Z 15 |  /  index_struct! {
2019-07-23T00:54:43.8108887Z 16 |  |      pub(crate) struct StackIndex {
2019-07-23T00:54:43.8109287Z 17 |  |          value: usize,
2019-07-23T00:54:43.8109607Z 18 |  |      }
2019-07-23T00:54:43.8109947Z 19 |  |  }
2019-07-23T00:54:43.8110276Z    |  |__- in this macro invocation
2019-07-23T00:54:43.8110406Z 
2019-07-23T00:54:43.8110756Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8111070Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-23T00:54:43.8111311Z    |
2019-07-23T00:54:43.8111948Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:43.8112332Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:43.8112681Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:43.8113310Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:43.8113582Z ...    |
2019-07-23T00:54:43.8113993Z 18 |   | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:43.8114395Z    |  _|_____________________________________________________________________________^
2019-07-23T00:54:43.8114925Z 19 | | | sub_one ( & self ) -> Self {
2019-07-23T00:54:43.8115339Z 20 | | | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:43.8115781Z    | |_|________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8116078Z ...    |
2019-07-23T00:54:43.8116406Z 23 |   | } impl From < usize > for $ n {
2019-07-23T00:54:43.8116797Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:43.8117197Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:43.8119099Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-23T00:54:43.8119475Z    |
2019-07-23T00:54:43.8119475Z    |
2019-07-23T00:54:43.8238889Z 15 |  /  index_struct! {
2019-07-23T00:54:43.8239365Z 16 |  |      pub(crate) struct StackIndex {
2019-07-23T00:54:43.8239710Z 17 |  |          value: usize,
2019-07-23T00:54:43.8240058Z 18 |  |      }
2019-07-23T00:54:43.8240610Z 19 |  |  }
2019-07-23T00:54:43.8240943Z    |  |__- in this macro invocation
2019-07-23T00:54:43.8241014Z 
2019-07-23T00:54:43.8241331Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8241635Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-23T00:54:43.8242071Z    |
2019-07-23T00:54:43.8242411Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:43.8242762Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:43.8243126Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:43.8243480Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:43.8243752Z ...    |
2019-07-23T00:54:43.8244095Z 20 |   | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:43.8244434Z    |  _|__________________________________________________________^
2019-07-23T00:54:43.8244797Z 21 | | | & self , n : usize ) -> Option < Self > {
2019-07-23T00:54:43.8245185Z 22 | | | usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-23T00:54:43.8245640Z    | |_|______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:43.8245966Z 23 |   | } impl From < usize > for $ n {
2019-07-23T00:54:43.8246328Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:43.8246697Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:43.8247360Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-07-23T00:54:43.8247791Z    |
2019-07-23T00:54:43.8247791Z    |
2019-07-23T00:54:43.8248082Z 15 | /   index_struct! {
2019-07-23T00:54:43.8248980Z 16 | |       pub(crate) struct StackIndex {
2019-07-23T00:54:43.8249311Z 17 | |           value: usize,
2019-07-23T00:54:43.8250124Z 19 | |   }
2019-07-23T00:54:43.8250447Z    | |___- in this macro invocation
2019-07-23T00:54:43.8250513Z 
2019-07-23T00:54:43.8250513Z 
2019-07-23T00:54:44.7729867Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7734035Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-23T00:54:44.7734250Z    |
2019-07-23T00:54:44.7734507Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7734801Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7735071Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7735363Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7735545Z ...   |
2019-07-23T00:54:44.7736279Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-23T00:54:44.7736600Z    |  |______________________________________________________________^
2019-07-23T00:54:44.7736845Z 14 | || & mut self ) -> Self {
2019-07-23T00:54:44.7737152Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:44.7737478Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7737691Z ...   |
2019-07-23T00:54:44.7737932Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7738199Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7739095Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7739916Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-23T00:54:44.7740348Z    |
2019-07-23T00:54:44.7740348Z    |
2019-07-23T00:54:44.7740688Z 34 | /  index_struct! {
2019-07-23T00:54:44.7741023Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-23T00:54:44.7741316Z 36 | |          value: usize,
2019-07-23T00:54:44.7742481Z 38 | |  }
2019-07-23T00:54:44.7742719Z    | |__- in this macro invocation
2019-07-23T00:54:44.7742760Z 
2019-07-23T00:54:44.7742760Z 
2019-07-23T00:54:44.7743007Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7743227Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-23T00:54:44.7743396Z    |
2019-07-23T00:54:44.7744018Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7744292Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7744571Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7744849Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7745033Z ...   |
2019-07-23T00:54:44.7745312Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:44.7745565Z    |  |__________________________________________________________________^
2019-07-23T00:54:44.7745832Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-23T00:54:44.7746127Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:44.7746446Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7746656Z ...   |
2019-07-23T00:54:44.7746895Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7747182Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7747483Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7747968Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-23T00:54:44.7748158Z    |
2019-07-23T00:54:44.7748158Z    |
2019-07-23T00:54:44.7749695Z 34 | /  index_struct! {
2019-07-23T00:54:44.7750117Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-23T00:54:44.7750414Z 36 | |          value: usize,
2019-07-23T00:54:44.7751150Z 38 | |  }
2019-07-23T00:54:44.7751447Z    | |__- in this macro invocation
2019-07-23T00:54:44.7751506Z 
2019-07-23T00:54:44.7751506Z 
2019-07-23T00:54:44.7752092Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7752311Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-23T00:54:44.7752680Z    |
2019-07-23T00:54:44.7753159Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7753480Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7753803Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7754279Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7754509Z ...   |
2019-07-23T00:54:44.7754811Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:44.7755100Z    |  |___________________________________________________________________^
2019-07-23T00:54:44.7755607Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:44.7756111Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7756334Z ...   |
2019-07-23T00:54:44.7756755Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7757055Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7757364Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7757887Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-23T00:54:44.7758087Z    |
2019-07-23T00:54:44.7758087Z    |
2019-07-23T00:54:44.7758870Z 34 | /  index_struct! {
2019-07-23T00:54:44.7759233Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-23T00:54:44.7759527Z 36 | |          value: usize,
2019-07-23T00:54:44.7760244Z 38 | |  }
2019-07-23T00:54:44.7760537Z    | |__- in this macro invocation
2019-07-23T00:54:44.7760594Z 
2019-07-23T00:54:44.7760594Z 
2019-07-23T00:54:44.7760875Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7761143Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-23T00:54:44.7761370Z    |
2019-07-23T00:54:44.7761711Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7762219Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7762569Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7762914Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7763165Z ...   |
2019-07-23T00:54:44.7764196Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:44.7764477Z    |  |_____________________________________________________________________________^
2019-07-23T00:54:44.7764760Z 19 | || sub_one ( & self ) -> Self {
2019-07-23T00:54:44.7765063Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:44.7765411Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7765621Z ...   |
2019-07-23T00:54:44.7765874Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7766180Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7766493Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7767008Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-23T00:54:44.7767229Z    |
2019-07-23T00:54:44.7767229Z    |
2019-07-23T00:54:44.7767684Z 34 | /  index_struct! {
2019-07-23T00:54:44.7767973Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-23T00:54:44.7768582Z 36 | |          value: usize,
2019-07-23T00:54:44.7769322Z 38 | |  }
2019-07-23T00:54:44.7769632Z    | |__- in this macro invocation
2019-07-23T00:54:44.7769679Z 
2019-07-23T00:54:44.7769679Z 
2019-07-23T00:54:44.7769953Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7770244Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-23T00:54:44.7770456Z    |
2019-07-23T00:54:44.7770795Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7771167Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7771510Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7772019Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7772243Z ...   |
2019-07-23T00:54:44.7772528Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:44.7772817Z    |  |__________________________________________________________^
2019-07-23T00:54:44.7773097Z 21 | || & self , n : usize ) -> Option < Self > {
2019-07-23T00:54:44.7773418Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-23T00:54:44.7773797Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7774221Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7774520Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7774834Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7775369Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-07-23T00:54:44.7775571Z    |
2019-07-23T00:54:44.7775571Z    |
2019-07-23T00:54:44.7775823Z 34 | /  index_struct! {
2019-07-23T00:54:44.7776072Z 35 | |      pub(crate) struct AnswerIndex {
2019-07-23T00:54:44.7776309Z 36 | |          value: usize,
2019-07-23T00:54:44.7776859Z 38 | |  }
2019-07-23T00:54:44.7777270Z    | |__- in this macro invocation
2019-07-23T00:54:44.7777308Z 
2019-07-23T00:54:44.7777308Z 
2019-07-23T00:54:44.7777526Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7777739Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-23T00:54:44.7778100Z    |
2019-07-23T00:54:44.7778782Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7779167Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7779508Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7779868Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7780130Z ...   |
2019-07-23T00:54:44.7780476Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-23T00:54:44.7780800Z    |  |______________________________________________________________^
2019-07-23T00:54:44.7781133Z 14 | || & mut self ) -> Self {
2019-07-23T00:54:44.7781498Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:44.7782121Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7782334Z ...   |
2019-07-23T00:54:44.7782610Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7782903Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7783245Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7783770Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-23T00:54:44.7784161Z    |
2019-07-23T00:54:44.7784161Z    |
2019-07-23T00:54:44.7784392Z 81 | /  index_struct! {
2019-07-23T00:54:44.7784652Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-23T00:54:44.7784911Z 83 | |          value: usize,
2019-07-23T00:54:44.7785599Z 85 | |  }
2019-07-23T00:54:44.7785855Z    | |__- in this macro invocation
2019-07-23T00:54:44.7785885Z 
2019-07-23T00:54:44.7785885Z 
2019-07-23T00:54:44.7786102Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7786334Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-23T00:54:44.7786500Z    |
2019-07-23T00:54:44.7786767Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7787065Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7787334Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7787636Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7787865Z ...   |
2019-07-23T00:54:44.7788139Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:44.7789113Z    |  |__________________________________________________________________^
2019-07-23T00:54:44.7789483Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-23T00:54:44.7789877Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:44.7790291Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7790539Z ...   |
2019-07-23T00:54:44.7790863Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7791209Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7791616Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7792633Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-23T00:54:44.7792821Z    |
2019-07-23T00:54:44.7792821Z    |
2019-07-23T00:54:44.7793225Z 81 | /  index_struct! {
2019-07-23T00:54:44.7793514Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-23T00:54:44.7793762Z 83 | |          value: usize,
2019-07-23T00:54:44.7794846Z 85 | |  }
2019-07-23T00:54:44.7795074Z    | |__- in this macro invocation
2019-07-23T00:54:44.7795104Z 
2019-07-23T00:54:44.7795104Z 
2019-07-23T00:54:44.7795333Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7795539Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-23T00:54:44.7795698Z    |
2019-07-23T00:54:44.7795978Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7796260Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7796533Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7796810Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7796995Z ...   |
2019-07-23T00:54:44.7797276Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:44.7797681Z    |  |___________________________________________________________________^
2019-07-23T00:54:44.7797956Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:44.7798467Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7799031Z ...   |
2019-07-23T00:54:44.7799367Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7799712Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7800119Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7800756Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-23T00:54:44.7801004Z    |
2019-07-23T00:54:44.7801004Z    |
2019-07-23T00:54:44.7801285Z 81 | /  index_struct! {
2019-07-23T00:54:44.7801624Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-23T00:54:44.7801921Z 83 | |          value: usize,
2019-07-23T00:54:44.7802649Z 85 | |  }
2019-07-23T00:54:44.7802882Z    | |__- in this macro invocation
2019-07-23T00:54:44.7802913Z 
2019-07-23T00:54:44.7802913Z 
2019-07-23T00:54:44.7803329Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7803550Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-23T00:54:44.7803719Z    |
2019-07-23T00:54:44.7804332Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7804607Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7804869Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7805165Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7805351Z ...   |
2019-07-23T00:54:44.7805633Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:44.7805896Z    |  |_____________________________________________________________________________^
2019-07-23T00:54:44.7806151Z 19 | || sub_one ( & self ) -> Self {
2019-07-23T00:54:44.7806604Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:44.7806896Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7807095Z ...   |
2019-07-23T00:54:44.7807317Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7824505Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7824869Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7825508Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-23T00:54:44.7825683Z    |
2019-07-23T00:54:44.7825683Z    |
2019-07-23T00:54:44.7825925Z 81 | /  index_struct! {
2019-07-23T00:54:44.7826260Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-23T00:54:44.7826689Z 83 | |          value: usize,
2019-07-23T00:54:44.7827253Z 85 | |  }
2019-07-23T00:54:44.7827481Z    | |__- in this macro invocation
2019-07-23T00:54:44.7827530Z 
2019-07-23T00:54:44.7827530Z 
2019-07-23T00:54:44.7827743Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7827950Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-07-23T00:54:44.7828130Z    |
2019-07-23T00:54:44.7828835Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7829214Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7829576Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7829930Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7830191Z ...   |
2019-07-23T00:54:44.7830537Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:44.7830856Z    |  |__________________________________________________________^
2019-07-23T00:54:44.7831218Z 21 | || & self , n : usize ) -> Option < Self > {
2019-07-23T00:54:44.7831596Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-07-23T00:54:44.7832350Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7832617Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7832897Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7833235Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7833810Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-07-23T00:54:44.7834347Z    |
2019-07-23T00:54:44.7834347Z    |
2019-07-23T00:54:44.7834564Z 81 | /  index_struct! {
2019-07-23T00:54:44.7834830Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-07-23T00:54:44.7835056Z 83 | |          value: usize,
2019-07-23T00:54:44.7835763Z 85 | |  }
2019-07-23T00:54:44.7835996Z    | |__- in this macro invocation
2019-07-23T00:54:44.7836288Z 
2019-07-23T00:54:44.7836288Z 
2019-07-23T00:54:44.7836744Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7837158Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-07-23T00:54:44.7837556Z    |
2019-07-23T00:54:44.7837853Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7838706Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7839119Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7839648Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7839935Z ...   |
2019-07-23T00:54:44.7840467Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-07-23T00:54:44.7840833Z    |  |______________________________________________________________^
2019-07-23T00:54:44.7841359Z 14 | || & mut self ) -> Self {
2019-07-23T00:54:44.7841746Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:44.7842595Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7842821Z ...   |
2019-07-23T00:54:44.7843073Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7843375Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7858751Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7859632Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-23T00:54:44.7859846Z    |
2019-07-23T00:54:44.7859846Z    |
2019-07-23T00:54:44.7860151Z 91 | /  index_struct! {
2019-07-23T00:54:44.7860444Z 92 | |      struct StackIndex {
2019-07-23T00:54:44.7860735Z 93 | |          value: usize,
2019-07-23T00:54:44.7861422Z 95 | |  }
2019-07-23T00:54:44.7861739Z    | |__- in this macro invocation
2019-07-23T00:54:44.7861781Z 
2019-07-23T00:54:44.7861781Z 
2019-07-23T00:54:44.7862278Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7862538Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-07-23T00:54:44.7862765Z    |
2019-07-23T00:54:44.7863093Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7863465Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7863945Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7864598Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7865061Z ...   |
2019-07-23T00:54:44.7865504Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-07-23T00:54:44.7865774Z    |  |__________________________________________________________________^
2019-07-23T00:54:44.7866072Z 16 | || replace_zero ( & mut self ) -> Self {
2019-07-23T00:54:44.7866378Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:44.7866734Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7866938Z ...   |
2019-07-23T00:54:44.7867203Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7867499Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7867891Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7868892Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-23T00:54:44.7869135Z    |
2019-07-23T00:54:44.7869135Z    |
2019-07-23T00:54:44.7869416Z 91 | /  index_struct! {
2019-07-23T00:54:44.7869711Z 92 | |      struct StackIndex {
2019-07-23T00:54:44.7870156Z 93 | |          value: usize,
2019-07-23T00:54:44.7870715Z 95 | |  }
2019-07-23T00:54:44.7871026Z    | |__- in this macro invocation
2019-07-23T00:54:44.7871064Z 
2019-07-23T00:54:44.7871064Z 
2019-07-23T00:54:44.7871333Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7871624Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-07-23T00:54:44.7871988Z    |
2019-07-23T00:54:44.7872459Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7872786Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7873068Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7873383Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7873587Z ...   |
2019-07-23T00:54:44.7874033Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-07-23T00:54:44.7874321Z    |  |___________________________________________________________________^
2019-07-23T00:54:44.7874628Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:44.7874991Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7875200Z ...   |
2019-07-23T00:54:44.7875449Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7875751Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7876067Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-07-23T00:54:44.7876629Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-07-23T00:54:44.7876877Z    |
2019-07-23T00:54:44.7876877Z    |
2019-07-23T00:54:44.7877128Z 91 | /  index_struct! {
2019-07-23T00:54:44.7877389Z 92 | |      struct StackIndex {
2019-07-23T00:54:44.7877712Z 93 | |          value: usize,
2019-07-23T00:54:44.7878305Z 95 | |  }
2019-07-23T00:54:44.7878978Z    | |__- in this macro invocation
2019-07-23T00:54:44.7879017Z 
2019-07-23T00:54:44.7879017Z 
2019-07-23T00:54:44.7879318Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7879616Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-07-23T00:54:44.7879829Z    |
2019-07-23T00:54:44.7880169Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-07-23T00:54:44.7880544Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-07-23T00:54:44.7880885Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-07-23T00:54:44.7881260Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-07-23T00:54:44.7881507Z ...   |
2019-07-23T00:54:44.7882075Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-07-23T00:54:44.7882388Z    |  |_____________________________________________________________________________^
2019-07-23T00:54:44.7882679Z 19 | || sub_one ( & self ) -> Self {
2019-07-23T00:54:44.7882997Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-07-23T00:54:44.7883366Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-07-23T00:54:44.7883580Z ...   |
2019-07-23T00:54:44.7884024Z 23 |  | } impl From < usize > for $ n {
2019-07-23T00:54:44.7884314Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-07-23T00:54:44.7884658Z    |  |________________________________________________________________- in this expansion of `index_struct!`
