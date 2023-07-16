plain
2019-08-18T19:06:08.3907595Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-18T19:06:08.4093068Z ##[command]git config gc.auto 0
2019-08-18T19:06:08.4172532Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-18T19:06:08.4225868Z ##[command]git config --get-all http.proxy
2019-08-18T19:06:08.4362421Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-08-18T19:06:43.9977141Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T19:06:43.9977355Z 
2019-08-18T19:06:43.9977742Z   git checkout -b <new-branch-name>
2019-08-18T19:06:43.9977942Z 
2019-08-18T19:06:43.9978130Z HEAD is now at 4f5351379 Merge ead9598216b4926bd3bf12c3b359943410e6514a into ea52be482ab4945fda63cb65b6a198309a041e3c
2019-08-18T19:06:44.0159794Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T19:06:44.0162323Z ==============================================================================
2019-08-18T19:06:44.0162372Z Task         : Bash
2019-08-18T19:06:44.0162428Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T19:13:02.2359129Z    Compiling rustc_version v0.2.3
2019-08-18T19:13:03.0692137Z     Checking lock_api v0.1.3
2019-08-18T19:13:03.6764108Z     Checking polonius-engine v0.9.0
2019-08-18T19:13:04.4975219Z     Checking chalk-engine v0.9.0
2019-08-18T19:13:04.5864686Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.5865093Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-18T19:13:04.5865566Z    |
2019-08-18T19:13:04.5865860Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.5866133Z 2  |   | {
2019-08-18T19:13:04.5866439Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.5866739Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.5866942Z ...    |
2019-08-18T19:13:04.5867643Z 18 |   |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-18T19:13:04.5868091Z    |  _|______________________________________________________________________^
2019-08-18T19:13:04.5868449Z 19 | | |         replace_one (& mut self) -> Self
2019-08-18T19:13:04.5868863Z 20 | | |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.5869294Z    | |_|________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.5869559Z ...    |
2019-08-18T19:13:04.5869936Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.5870240Z 34 |   | }
2019-08-18T19:13:04.5870587Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.5871254Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-18T19:13:04.5871455Z    |
2019-08-18T19:13:04.5871455Z    |
2019-08-18T19:13:04.5871704Z 15 |  /  index_struct! {
2019-08-18T19:13:04.5871981Z 16 |  |      pub(crate) struct StackIndex {
2019-08-18T19:13:04.5872256Z 17 |  |          value: usize,
2019-08-18T19:13:04.5872766Z 19 |  |  }
2019-08-18T19:13:04.5873305Z    |  |__- in this macro invocation
2019-08-18T19:13:04.5885449Z 
2019-08-18T19:13:04.5885449Z 
2019-08-18T19:13:04.5885955Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.5886200Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-18T19:13:04.5896518Z    |
2019-08-18T19:13:04.5897681Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.5898112Z 2  |   | {
2019-08-18T19:13:04.5898532Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.5898928Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.5899209Z ...    |
2019-08-18T19:13:04.5899597Z 20 |   |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.5900041Z    |  _|__________________________________________________________________________^
2019-08-18T19:13:04.5900420Z 21 | | |         replace_zero (& mut self) -> Self
2019-08-18T19:13:04.5901100Z 22 | | |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.5901642Z    | |_|_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.5901907Z ...    |
2019-08-18T19:13:04.5902408Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.5902696Z 34 |   | }
2019-08-18T19:13:04.5902989Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.5903512Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-18T19:13:04.5903718Z    |
2019-08-18T19:13:04.5903718Z    |
2019-08-18T19:13:04.5903982Z 15 | /   index_struct! {
2019-08-18T19:13:04.5904307Z 16 | |       pub(crate) struct StackIndex {
2019-08-18T19:13:04.5904585Z 17 | |           value: usize,
2019-08-18T19:13:04.5905130Z 19 | |   }
2019-08-18T19:13:04.5905733Z    | |___- in this macro invocation
2019-08-18T19:13:04.5905798Z 
2019-08-18T19:13:04.5905798Z 
2019-08-18T19:13:04.5988100Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.5988620Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-18T19:13:04.5989112Z    |
2019-08-18T19:13:04.5989468Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.5989764Z 2  |   | {
2019-08-18T19:13:04.5990168Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.5990519Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.5990954Z ...    |
2019-08-18T19:13:04.5991426Z 22 |   |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.5991889Z    |  _|___________________________________________________________________________^
2019-08-18T19:13:04.5992182Z 23 | | |         add_one (& self) -> Self
2019-08-18T19:13:04.5992492Z 24 | | |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.5992848Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.5993062Z ...    |
2019-08-18T19:13:04.5993357Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.5993601Z 34 |   | }
2019-08-18T19:13:04.5993862Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.5994312Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-18T19:13:04.5994492Z    |
2019-08-18T19:13:04.5994492Z    |
2019-08-18T19:13:04.5994740Z 15 | /   index_struct! {
2019-08-18T19:13:04.5995005Z 16 | |       pub(crate) struct StackIndex {
2019-08-18T19:13:04.5995266Z 17 | |           value: usize,
2019-08-18T19:13:04.5995721Z 19 | |   }
2019-08-18T19:13:04.5996060Z    | |___- in this macro invocation
2019-08-18T19:13:04.5996104Z 
2019-08-18T19:13:04.5996104Z 
2019-08-18T19:13:04.6061890Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6062236Z   --> <::chalk_macros::index::index_struct macros>:24:66
2019-08-18T19:13:04.6062673Z    |
2019-08-18T19:13:04.6063281Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6063541Z 2  |   | {
2019-08-18T19:13:04.6063829Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6064129Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6064333Z ...    |
2019-08-18T19:13:04.6064623Z 24 |   |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.6064942Z    |  _|__________________________________________________________________^
2019-08-18T19:13:04.6065209Z 25 | | |         (& self) -> Self
2019-08-18T19:13:04.6065548Z 26 | | |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-18T19:13:04.6065884Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6066103Z ...    |
2019-08-18T19:13:04.6066392Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6066646Z 34 |   | }
2019-08-18T19:13:04.6066897Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6067829Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-18T19:13:04.6068351Z    |
2019-08-18T19:13:04.6068351Z    |
2019-08-18T19:13:04.6068650Z 15 | /   index_struct! {
2019-08-18T19:13:04.6068986Z 16 | |       pub(crate) struct StackIndex {
2019-08-18T19:13:04.6069299Z 17 | |           value: usize,
2019-08-18T19:13:04.6069883Z 19 | |   }
2019-08-18T19:13:04.6070178Z    | |___- in this macro invocation
2019-08-18T19:13:04.6088808Z 
2019-08-18T19:13:04.6089620Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6089620Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6092932Z   --> <::chalk_macros::index::index_struct macros>:26:66
2019-08-18T19:13:04.6093197Z    |
2019-08-18T19:13:04.6093757Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6094223Z 2  |   | {
2019-08-18T19:13:04.6094597Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6094958Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6095201Z ...    |
2019-08-18T19:13:04.6095537Z 26 |   |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-18T19:13:04.6095914Z    |  _|__________________________________________________________________^
2019-08-18T19:13:04.6096251Z 27 | | |         (& self , n : usize) -> Option < Self >
2019-08-18T19:13:04.6096655Z 28 | | |         {
2019-08-18T19:13:04.6097620Z 29 | | |             usize :: add_usize (& self . value , n) . map
2019-08-18T19:13:04.6098041Z 30 | | |             (| value | Self { value })
2019-08-18T19:13:04.6098822Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6098822Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6099194Z 32 |   |     } impl From < usize > for $ n
2019-08-18T19:13:04.6099590Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6099915Z 34 |   | }
2019-08-18T19:13:04.6100282Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6101201Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-18T19:13:04.6101431Z    |
2019-08-18T19:13:04.6101431Z    |
2019-08-18T19:13:04.6101700Z 15 | /   index_struct! {
2019-08-18T19:13:04.6101992Z 16 | |       pub(crate) struct StackIndex {
2019-08-18T19:13:04.6102416Z 17 | |           value: usize,
2019-08-18T19:13:04.6103022Z 19 | |   }
2019-08-18T19:13:04.6103407Z    | |___- in this macro invocation
2019-08-18T19:13:04.6103448Z 
2019-08-18T19:13:04.6103448Z 
2019-08-18T19:13:04.6260380Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6262474Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-18T19:13:04.6263009Z    |
2019-08-18T19:13:04.6264695Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6265010Z 2  |   | {
2019-08-18T19:13:04.6265367Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6266149Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6266604Z ...    |
2019-08-18T19:13:04.6267977Z 18 |   |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-18T19:13:04.6269801Z    |  _|______________________________________________________________________^
2019-08-18T19:13:04.6270290Z 19 | | |         replace_one (& mut self) -> Self
2019-08-18T19:13:04.6273001Z 20 | | |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.6273645Z    | |_|________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6274216Z ...    |
2019-08-18T19:13:04.6274552Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6275051Z 34 |   | }
2019-08-18T19:13:04.6275560Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6276672Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-18T19:13:04.6278044Z    |
2019-08-18T19:13:04.6278044Z    |
2019-08-18T19:13:04.6278474Z 34 | /   index_struct! {
2019-08-18T19:13:04.6279080Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-18T19:13:04.6282122Z 36 | |           value: usize,
2019-08-18T19:13:04.6282982Z 38 | |   }
2019-08-18T19:13:04.6283406Z    | |___- in this macro invocation
2019-08-18T19:13:04.6283538Z 
2019-08-18T19:13:04.6283538Z 
2019-08-18T19:13:04.6283812Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6284055Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-18T19:13:04.6284270Z    |
2019-08-18T19:13:04.6284571Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6284838Z 2  |   | {
2019-08-18T19:13:04.6285177Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6285490Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6285729Z ...    |
2019-08-18T19:13:04.6286213Z 20 |   |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.6286719Z    |  _|__________________________________________________________________________^
2019-08-18T19:13:04.6287047Z 21 | | |         replace_zero (& mut self) -> Self
2019-08-18T19:13:04.6288199Z 22 | | |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.6288945Z    | |_|_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6289311Z ...    |
2019-08-18T19:13:04.6290010Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6290512Z 34 |   | }
2019-08-18T19:13:04.6291297Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6336253Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-18T19:13:04.6336502Z    |
2019-08-18T19:13:04.6336502Z    |
2019-08-18T19:13:04.6336760Z 34 | /   index_struct! {
2019-08-18T19:13:04.6337038Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-18T19:13:04.6338006Z 36 | |           value: usize,
2019-08-18T19:13:04.6338713Z 38 | |   }
2019-08-18T19:13:04.6339071Z    | |___- in this macro invocation
2019-08-18T19:13:04.6339213Z 
2019-08-18T19:13:04.6339213Z 
2019-08-18T19:13:04.6339550Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6339882Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-18T19:13:04.6340130Z    |
2019-08-18T19:13:04.6340503Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6341023Z 2  |   | {
2019-08-18T19:13:04.6341336Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6341660Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6341885Z ...    |
2019-08-18T19:13:04.6342188Z 22 |   |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.6342534Z    |  _|___________________________________________________________________________^
2019-08-18T19:13:04.6342832Z 23 | | |         add_one (& self) -> Self
2019-08-18T19:13:04.6343520Z 24 | | |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.6343882Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6344287Z ...    |
2019-08-18T19:13:04.6344597Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6344857Z 34 |   | }
2019-08-18T19:13:04.6345128Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6345617Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-18T19:13:04.6346162Z    |
2019-08-18T19:13:04.6346162Z    |
2019-08-18T19:13:04.6346455Z 34 | /   index_struct! {
2019-08-18T19:13:04.6346920Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-18T19:13:04.6347223Z 36 | |           value: usize,
2019-08-18T19:13:04.6348433Z 38 | |   }
2019-08-18T19:13:04.6348785Z    | |___- in this macro invocation
2019-08-18T19:13:04.6348916Z 
2019-08-18T19:13:04.6348916Z 
2019-08-18T19:13:04.6349343Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6349677Z   --> <::chalk_macros::index::index_struct macros>:24:66
2019-08-18T19:13:04.6349925Z    |
2019-08-18T19:13:04.6350295Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6350651Z 2  |   | {
2019-08-18T19:13:04.6351176Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6351478Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6351740Z ...    |
2019-08-18T19:13:04.6352055Z 24 |   |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.6352559Z    |  _|__________________________________________________________________^
2019-08-18T19:13:04.6353030Z 25 | | |         (& self) -> Self
2019-08-18T19:13:04.6353513Z 26 | | |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-18T19:13:04.6353883Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6354091Z ...    |
2019-08-18T19:13:04.6354380Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6354624Z 34 |   | }
2019-08-18T19:13:04.6354877Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6355522Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-18T19:13:04.6355728Z    |
2019-08-18T19:13:04.6355728Z    |
2019-08-18T19:13:04.6355992Z 34 | /   index_struct! {
2019-08-18T19:13:04.6356258Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-18T19:13:04.6356512Z 36 | |           value: usize,
2019-08-18T19:13:04.6357133Z 38 | |   }
2019-08-18T19:13:04.6357855Z    | |___- in this macro invocation
2019-08-18T19:13:04.6358012Z 
2019-08-18T19:13:04.6465258Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6465258Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6465815Z   --> <::chalk_macros::index::index_struct macros>:26:66
2019-08-18T19:13:04.6466031Z    |
2019-08-18T19:13:04.6466543Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6466864Z 2  |   | {
2019-08-18T19:13:04.6467628Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6468109Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6469947Z ...    |
2019-08-18T19:13:04.6470634Z 26 |   |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-18T19:13:04.6471554Z    |  _|__________________________________________________________________^
2019-08-18T19:13:04.6473020Z 27 | | |         (& self , n : usize) -> Option < Self >
2019-08-18T19:13:04.6473551Z 28 | | |         {
2019-08-18T19:13:04.6473921Z 29 | | |             usize :: add_usize (& self . value , n) . map
2019-08-18T19:13:04.6474461Z 30 | | |             (| value | Self { value })
2019-08-18T19:13:04.6476734Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6476734Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6477288Z 32 |   |     } impl From < usize > for $ n
2019-08-18T19:13:04.6478963Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6479400Z 34 |   | }
2019-08-18T19:13:04.6479778Z    |   |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6480373Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-18T19:13:04.6480643Z    |
2019-08-18T19:13:04.6480643Z    |
2019-08-18T19:13:04.6481266Z 34 | /   index_struct! {
2019-08-18T19:13:04.6481625Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-18T19:13:04.6481932Z 36 | |           value: usize,
2019-08-18T19:13:04.6482751Z 38 | |   }
2019-08-18T19:13:04.6483206Z    | |___- in this macro invocation
2019-08-18T19:13:04.6483241Z 
2019-08-18T19:13:04.6483241Z 
2019-08-18T19:13:04.6483558Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6483831Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-18T19:13:04.6484024Z    |
2019-08-18T19:13:04.6484326Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6484591Z 2  |  | {
2019-08-18T19:13:04.6484890Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6485193Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6485406Z ...   |
2019-08-18T19:13:04.6485704Z 18 |  |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-18T19:13:04.6486198Z    |  |______________________________________________________________________^
2019-08-18T19:13:04.6486484Z 19 | ||         replace_one (& mut self) -> Self
2019-08-18T19:13:04.6486992Z 20 | ||         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.6487681Z    | ||________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6487970Z ...   |
2019-08-18T19:13:04.6488368Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6488704Z 34 |  | }
2019-08-18T19:13:04.6489069Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6489653Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-18T19:13:04.6489919Z    |
2019-08-18T19:13:04.6489919Z    |
2019-08-18T19:13:04.6490335Z 81 | /  index_struct! {
2019-08-18T19:13:04.6490740Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-18T19:13:04.6491212Z 83 | |          value: usize,
2019-08-18T19:13:04.6491818Z 85 | |  }
2019-08-18T19:13:04.6492098Z    | |__- in this macro invocation
2019-08-18T19:13:04.6492133Z 
2019-08-18T19:13:04.6492133Z 
2019-08-18T19:13:04.6492376Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6492811Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-18T19:13:04.6493007Z    |
2019-08-18T19:13:04.6493291Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6493560Z 2  |  | {
2019-08-18T19:13:04.6493869Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6494296Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6494552Z ...   |
2019-08-18T19:13:04.6494855Z 20 |  |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.6495165Z    |  |__________________________________________________________________________^
2019-08-18T19:13:04.6495638Z 21 | ||         replace_zero (& mut self) -> Self
2019-08-18T19:13:04.6495944Z 22 | ||         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.6496298Z    | ||_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6496514Z ...   |
2019-08-18T19:13:04.6496807Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6497244Z 34 |  | }
2019-08-18T19:13:04.6497861Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6498467Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-18T19:13:04.6498716Z    |
2019-08-18T19:13:04.6498716Z    |
2019-08-18T19:13:04.6499057Z 81 | /  index_struct! {
2019-08-18T19:13:04.6499541Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-18T19:13:04.6499925Z 83 | |          value: usize,
2019-08-18T19:13:04.6500692Z 85 | |  }
2019-08-18T19:13:04.6501205Z    | |__- in this macro invocation
2019-08-18T19:13:04.6501240Z 
2019-08-18T19:13:04.6501240Z 
2019-08-18T19:13:04.6623587Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6624290Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-18T19:13:04.6624729Z    |
2019-08-18T19:13:04.6625243Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6625538Z 2  |  | {
2019-08-18T19:13:04.6626071Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6628494Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6628906Z ...   |
2019-08-18T19:13:04.6629469Z 22 |  |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.6630052Z    |  |___________________________________________________________________________^
2019-08-18T19:13:04.6631191Z 23 | ||         add_one (& self) -> Self
2019-08-18T19:13:04.6631870Z 24 | ||         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.6632411Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6632718Z ...   |
2019-08-18T19:13:04.6633259Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6633515Z 34 |  | }
2019-08-18T19:13:04.6633804Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6634251Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-18T19:13:04.6634452Z    |
2019-08-18T19:13:04.6634452Z    |
2019-08-18T19:13:04.6634693Z 81 | /  index_struct! {
2019-08-18T19:13:04.6635116Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-18T19:13:04.6635450Z 83 | |          value: usize,
2019-08-18T19:13:04.6636408Z 85 | |  }
2019-08-18T19:13:04.6636660Z    | |__- in this macro invocation
2019-08-18T19:13:04.6636696Z 
2019-08-18T19:13:04.6636696Z 
2019-08-18T19:13:04.6637014Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6637616Z   --> <::chalk_macros::index::index_struct macros>:24:66
2019-08-18T19:13:04.6637867Z    |
2019-08-18T19:13:04.6638235Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6638528Z 2  |  | {
2019-08-18T19:13:04.6638881Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6639253Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6639498Z ...   |
2019-08-18T19:13:04.6639877Z 24 |  |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.6640221Z    |  |__________________________________________________________________^
2019-08-18T19:13:04.6640564Z 25 | ||         (& self) -> Self
2019-08-18T19:13:04.6641098Z 26 | ||         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-18T19:13:04.6641475Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6641868Z ...   |
2019-08-18T19:13:04.6642166Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6642429Z 34 |  | }
2019-08-18T19:13:04.6642693Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6643160Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-18T19:13:04.6643341Z    |
2019-08-18T19:13:04.6643341Z    |
2019-08-18T19:13:04.6643597Z 81 | /  index_struct! {
2019-08-18T19:13:04.6643872Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-18T19:13:04.6644226Z 83 | |          value: usize,
2019-08-18T19:13:04.6644763Z 85 | |  }
2019-08-18T19:13:04.6645265Z    | |__- in this macro invocation
2019-08-18T19:13:04.6645314Z 
2019-08-18T19:13:04.6730467Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6730467Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6733772Z   --> <::chalk_macros::index::index_struct macros>:26:66
2019-08-18T19:13:04.6734870Z    |
2019-08-18T19:13:04.6735232Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6735497Z 2  |  | {
2019-08-18T19:13:04.6735973Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6736450Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6736650Z ...   |
2019-08-18T19:13:04.6736953Z 26 |  |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-18T19:13:04.6737963Z    |  |__________________________________________________________________^
2019-08-18T19:13:04.6738327Z 27 | ||         (& self , n : usize) -> Option < Self >
2019-08-18T19:13:04.6738662Z 28 | ||         {
2019-08-18T19:13:04.6739024Z 29 | ||             usize :: add_usize (& self . value , n) . map
2019-08-18T19:13:04.6739379Z 30 | ||             (| value | Self { value })
2019-08-18T19:13:04.6739686Z 31 | ||         }
2019-08-18T19:13:04.6740049Z    | ||_________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6740445Z 32 |  |     } impl From < usize > for $ n
2019-08-18T19:13:04.6741028Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6741440Z 34 |  | }
2019-08-18T19:13:04.6741751Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6742219Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-18T19:13:04.6742553Z    |
2019-08-18T19:13:04.6742553Z    |
2019-08-18T19:13:04.6742839Z 81 | /  index_struct! {
2019-08-18T19:13:04.6743154Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-18T19:13:04.6743513Z 83 | |          value: usize,
2019-08-18T19:13:04.6744022Z 85 | |  }
2019-08-18T19:13:04.6744261Z    | |__- in this macro invocation
2019-08-18T19:13:04.6744332Z 
2019-08-18T19:13:04.6744332Z 
2019-08-18T19:13:04.6821528Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6821843Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-18T19:13:04.6848168Z    |
2019-08-18T19:13:04.6848770Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6849114Z 2  |  | {
2019-08-18T19:13:04.6849511Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6849855Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6850126Z ...   |
2019-08-18T19:13:04.6850488Z 18 |  |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-18T19:13:04.6850835Z    |  |______________________________________________________________________^
2019-08-18T19:13:04.6851362Z 19 | ||         replace_one (& mut self) -> Self
2019-08-18T19:13:04.6851722Z 20 | ||         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.6852286Z    | ||________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6852516Z ...   |
2019-08-18T19:13:04.6852829Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6853305Z 34 |  | }
2019-08-18T19:13:04.6853610Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6854372Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-18T19:13:04.6854584Z    |
2019-08-18T19:13:04.6854584Z    |
2019-08-18T19:13:04.6855048Z 91 | /  index_struct! {
2019-08-18T19:13:04.6855664Z 93 | |          value: usize,
2019-08-18T19:13:04.6856059Z 94 | |      }
2019-08-18T19:13:04.6856321Z 95 | |  }
2019-08-18T19:13:04.6856595Z    | |__- in this macro invocation
2019-08-18T19:13:04.6856595Z    | |__- in this macro invocation
2019-08-18T19:13:04.6899444Z 
2019-08-18T19:13:04.6911093Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6920521Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-18T19:13:04.6921012Z    |
2019-08-18T19:13:04.6921363Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6921640Z 2  |  | {
2019-08-18T19:13:04.6922333Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6922678Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6922917Z ...   |
2019-08-18T19:13:04.6923288Z 20 |  |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-18T19:13:04.6928941Z    |  |__________________________________________________________________________^
2019-08-18T19:13:04.6929401Z 21 | ||         replace_zero (& mut self) -> Self
2019-08-18T19:13:04.6929791Z 22 | ||         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.6930243Z    | ||_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6930500Z ...   |
2019-08-18T19:13:04.6931025Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.6931304Z 34 |  | }
2019-08-18T19:13:04.6931590Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.6932078Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-18T19:13:04.6932276Z    |
2019-08-18T19:13:04.6932276Z    |
2019-08-18T19:13:04.6932712Z 91 | /  index_struct! {
2019-08-18T19:13:04.6937925Z 93 | |          value: usize,
2019-08-18T19:13:04.6943581Z 94 | |      }
2019-08-18T19:13:04.6950220Z 95 | |  }
2019-08-18T19:13:04.6950603Z    | |__- in this macro invocation
2019-08-18T19:13:04.6950603Z    | |__- in this macro invocation
2019-08-18T19:13:04.6950672Z 
2019-08-18T19:13:04.6950957Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-18T19:13:04.6951250Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-18T19:13:04.6955936Z    |
2019-08-18T19:13:04.6956308Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-18T19:13:04.6961208Z 2  |  | {
2019-08-18T19:13:04.6961646Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-18T19:13:04.6966212Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-18T19:13:04.6970676Z ...   |
2019-08-18T19:13:04.6971324Z 22 |  |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-18T19:13:04.6971636Z    |  |___________________________________________________________________________^
2019-08-18T19:13:04.7054759Z 23 | ||         add_one (& self) -> Self
2019-08-18T19:13:04.7055216Z 24 | ||         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-18T19:13:04.7055793Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-18T19:13:04.7056036Z ...   |
2019-08-18T19:13:04.7056355Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-18T19:13:04.7056646Z 34 |  | }
2019-08-18T19:13:04.7057580Z    |  |_- in this expansion of `index_struct!`
2019-08-18T19:13:04.7058180Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-18T19:13:04.7058398Z    |
2019-08-18T19:13:04.7058398Z    |
2019-08-18T19:13:04.7058689Z 91 | /  index_struct! {
2019-08-18T19:13:04.7059565Z 93 | |          value: usize,
2019-08-18T19:13:04.7059869Z 94 | |      }
2019-08-18T19:13:04.7060267Z 95 | |  }
2019-08-18T19:13:04.7060881Z    | |__- in this macro invocation
2019-08-18T19:13:04.7060881Z    | |__- in this macro invocation
2019-08-18T19:13:04.7060920Z 
2019-08-18T19:13:04.7081602Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
---
2019-08-18T19:13:06.7472071Z == clock drift check ==
2019-08-18T19:13:06.7487117Z   local time: Sun Aug 18 19:13:06 UTC 2019
2019-08-18T19:13:06.8995023Z   network time: Sun, 18 Aug 2019 19:13:06 GMT
2019-08-18T19:13:06.8997170Z == end clock drift check ==
2019-08-18T19:13:09.6749115Z ##[error]Bash exited with code '1'.
2019-08-18T19:13:09.6780171Z ##[section]Starting: Checkout
2019-08-18T19:13:09.6781961Z ==============================================================================
2019-08-18T19:13:09.6782024Z Task         : Get sources
2019-08-18T19:13:09.6782064Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
