plain
2020-02-03T21:25:27.0673094Z ========================== Starting Command Output ===========================
2020-02-03T21:25:27.0687089Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/121d8d7d-d2e6-448e-b726-b406bb45656c.sh
2020-02-03T21:25:27.0863670Z 
2020-02-03T21:25:27.0914107Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-03T21:25:27.0918772Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68807/merge to s
2020-02-03T21:25:27.0920132Z Task         : Get sources
2020-02-03T21:25:27.0920161Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-03T21:25:27.0920188Z Version      : 1.0.0
2020-02-03T21:25:27.0920216Z Author       : Microsoft
---
2020-02-03T21:25:28.5452529Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-03T21:25:28.5527114Z ##[command]git config gc.auto 0
2020-02-03T21:25:28.5589642Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-03T21:25:28.5634061Z ##[command]git config --get-all http.proxy
2020-02-03T21:25:28.5746940Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68807/merge:refs/remotes/pull/68807/merge
---
2020-02-03T21:30:08.5790365Z     Checking chalk-engine v0.9.0
2020-02-03T21:30:08.6507404Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6510718Z   --> <::chalk_macros::index::index_struct macros>:18:69
2020-02-03T21:30:08.6510982Z    |
2020-02-03T21:30:08.6511246Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6515890Z 2  |   | {
2020-02-03T21:30:08.6517043Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6517377Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6517600Z ...    |
2020-02-03T21:30:08.6524626Z 18 |   |         { usize :: steps_between (& start . value, & end . value) } fn
2020-02-03T21:30:08.6524999Z    |  _|_____________________________________________________________________^
2020-02-03T21:30:08.6525385Z 19 | | |         replace_one (& mut self) -> Self
2020-02-03T21:30:08.6525690Z 20 | | |         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.6526025Z    | |_|_______________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6526241Z ...    |
2020-02-03T21:30:08.6526506Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6526997Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6527188Z    | 
2020-02-03T21:30:08.6527438Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2020-02-03T21:30:08.6527635Z    |
2020-02-03T21:30:08.6527635Z    |
2020-02-03T21:30:08.6527883Z 15 |  /  index_struct! {
2020-02-03T21:30:08.6528162Z 16 |  |      pub(crate) struct StackIndex {
2020-02-03T21:30:08.6528667Z 18 |  |      }
2020-02-03T21:30:08.6528919Z 19 |  |  }
2020-02-03T21:30:08.6529171Z    |  |__- in this macro invocation
2020-02-03T21:30:08.6529202Z 
2020-02-03T21:30:08.6529202Z 
2020-02-03T21:30:08.6529442Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6529672Z   --> <::chalk_macros::index::index_struct macros>:20:73
2020-02-03T21:30:08.6529850Z    |
2020-02-03T21:30:08.6530141Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6530392Z 2  |   | {
2020-02-03T21:30:08.6530684Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6530974Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6531173Z ...    |
2020-02-03T21:30:08.6531527Z 20 |   |         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.6531866Z    |  _|_________________________________________________________________________^
2020-02-03T21:30:08.6532231Z 21 | | |         replace_zero (& mut self) -> Self
2020-02-03T21:30:08.6532562Z 22 | | |         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.6532913Z    | |_|________________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6533138Z ...    |
2020-02-03T21:30:08.6533416Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6533946Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6534122Z    | 
2020-02-03T21:30:08.6534383Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2020-02-03T21:30:08.6534562Z    |
2020-02-03T21:30:08.6534562Z    |
2020-02-03T21:30:08.6534804Z 15 | /   index_struct! {
2020-02-03T21:30:08.6535074Z 16 | |       pub(crate) struct StackIndex {
2020-02-03T21:30:08.6535562Z 18 | |       }
2020-02-03T21:30:08.6535810Z 19 | |   }
2020-02-03T21:30:08.6536049Z    | |___- in this macro invocation
2020-02-03T21:30:08.6536082Z 
2020-02-03T21:30:08.6536082Z 
2020-02-03T21:30:08.6536322Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6536551Z   --> <::chalk_macros::index::index_struct macros>:22:74
2020-02-03T21:30:08.6536725Z    |
2020-02-03T21:30:08.6537015Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6537267Z 2  |   | {
2020-02-03T21:30:08.6537557Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6538024Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6538229Z ...    |
2020-02-03T21:30:08.6538780Z 22 |   |         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.6539107Z    |  _|__________________________________________________________________________^
2020-02-03T21:30:08.6539489Z 23 | | |         add_one (& self) -> Self
2020-02-03T21:30:08.6561158Z 24 | | |         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.6561612Z    | |_|_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6561841Z ...    |
2020-02-03T21:30:08.6562105Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6562599Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6562802Z    | 
2020-02-03T21:30:08.6563052Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2020-02-03T21:30:08.6563230Z    |
2020-02-03T21:30:08.6563230Z    |
2020-02-03T21:30:08.6563490Z 15 | /   index_struct! {
2020-02-03T21:30:08.6563916Z 16 | |       pub(crate) struct StackIndex {
2020-02-03T21:30:08.6564783Z 18 | |       }
2020-02-03T21:30:08.6565024Z 19 | |   }
2020-02-03T21:30:08.6565282Z    | |___- in this macro invocation
2020-02-03T21:30:08.6565333Z 
2020-02-03T21:30:08.6565333Z 
2020-02-03T21:30:08.6565637Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6565884Z   --> <::chalk_macros::index::index_struct macros>:24:65
2020-02-03T21:30:08.6566093Z    |
2020-02-03T21:30:08.6566390Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6566643Z 2  |   | {
2020-02-03T21:30:08.6566991Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6567279Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6567513Z ...    |
2020-02-03T21:30:08.6568204Z 24 |   |         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.6568591Z    |  _|_________________________________________________________________^
2020-02-03T21:30:08.6568896Z 25 | | |         (& self) -> Self
2020-02-03T21:30:08.6569337Z 26 | | |         { Self { value : usize :: sub_one (& self . value), } } fn add_usize
2020-02-03T21:30:08.6569922Z    | |_|_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6570167Z ...    |
2020-02-03T21:30:08.6570512Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6571313Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6571522Z    | 
2020-02-03T21:30:08.6571974Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2020-02-03T21:30:08.6572193Z    |
2020-02-03T21:30:08.6572193Z    |
2020-02-03T21:30:08.6572460Z 15 | /   index_struct! {
2020-02-03T21:30:08.6572756Z 16 | |       pub(crate) struct StackIndex {
2020-02-03T21:30:08.6573659Z 18 | |       }
2020-02-03T21:30:08.6573911Z 19 | |   }
2020-02-03T21:30:08.6574706Z    | |___- in this macro invocation
2020-02-03T21:30:08.6574738Z 
2020-02-03T21:30:08.6574738Z 
2020-02-03T21:30:08.6665295Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6665566Z   --> <::chalk_macros::index::index_struct macros>:26:65
2020-02-03T21:30:08.6665743Z    |
2020-02-03T21:30:08.6666023Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6666246Z 2  |   | {
2020-02-03T21:30:08.6666530Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6666805Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6666993Z ...    |
2020-02-03T21:30:08.6667399Z 26 |   |         { Self { value : usize :: sub_one (& self . value), } } fn add_usize
2020-02-03T21:30:08.6667713Z    |  _|_________________________________________________________________^
2020-02-03T21:30:08.6668002Z 27 | | |         (& self, n : usize) -> Option < Self >
2020-02-03T21:30:08.6668336Z 28 | | |         {
2020-02-03T21:30:08.6668610Z 29 | | |             usize :: add_usize (& self . value, n) . map
2020-02-03T21:30:08.6668894Z 30 | | |             (| value | Self { value })
2020-02-03T21:30:08.6669433Z    | |_|_________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6669433Z    | |_|_________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6669691Z 32 |   |     } impl From < usize > for $ n
2020-02-03T21:30:08.6669955Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6670438Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6670632Z    | 
2020-02-03T21:30:08.6670904Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2020-02-03T21:30:08.6671083Z    |
2020-02-03T21:30:08.6671083Z    |
2020-02-03T21:30:08.6671319Z 15 | /   index_struct! {
2020-02-03T21:30:08.6671601Z 16 | |       pub(crate) struct StackIndex {
2020-02-03T21:30:08.6672094Z 18 | |       }
2020-02-03T21:30:08.6672336Z 19 | |   }
2020-02-03T21:30:08.6672577Z    | |___- in this macro invocation
2020-02-03T21:30:08.6689171Z 
2020-02-03T21:30:08.6689171Z 
2020-02-03T21:30:08.6689587Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6689808Z   --> <::chalk_macros::index::index_struct macros>:18:69
2020-02-03T21:30:08.6690009Z    |
2020-02-03T21:30:08.6690274Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6690494Z 2  |   | {
2020-02-03T21:30:08.6690893Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6691178Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6691364Z ...    |
2020-02-03T21:30:08.6691651Z 18 |   |         { usize :: steps_between (& start . value, & end . value) } fn
2020-02-03T21:30:08.6692008Z    |  _|_____________________________________________________________________^
2020-02-03T21:30:08.6692293Z 19 | | |         replace_one (& mut self) -> Self
2020-02-03T21:30:08.6692591Z 20 | | |         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.6692914Z    | |_|_______________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6693142Z ...    |
2020-02-03T21:30:08.6693408Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6693896Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6694284Z    | 
2020-02-03T21:30:08.6694539Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2020-02-03T21:30:08.6694891Z    |
2020-02-03T21:30:08.6694891Z    |
2020-02-03T21:30:08.6695144Z 34 | /   index_struct! {
2020-02-03T21:30:08.6695586Z 35 | |       pub(crate) struct AnswerIndex {
2020-02-03T21:30:08.6696287Z 37 | |       }
2020-02-03T21:30:08.6696531Z 38 | |   }
2020-02-03T21:30:08.6696795Z    | |___- in this macro invocation
2020-02-03T21:30:08.6696846Z 
2020-02-03T21:30:08.6696846Z 
2020-02-03T21:30:08.6784551Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6784790Z   --> <::chalk_macros::index::index_struct macros>:20:73
2020-02-03T21:30:08.6784995Z    |
2020-02-03T21:30:08.6785256Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6785477Z 2  |   | {
2020-02-03T21:30:08.6785773Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6786137Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6786385Z ...    |
2020-02-03T21:30:08.6786661Z 20 |   |         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.6787044Z    |  _|_________________________________________________________________________^
2020-02-03T21:30:08.6787318Z 21 | | |         replace_zero (& mut self) -> Self
2020-02-03T21:30:08.6787634Z 22 | | |         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.6787960Z    | |_|________________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6788164Z ...    |
2020-02-03T21:30:08.6788453Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6788941Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6789138Z    | 
2020-02-03T21:30:08.6789391Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2020-02-03T21:30:08.6789588Z    |
2020-02-03T21:30:08.6789588Z    |
2020-02-03T21:30:08.6789826Z 34 | /   index_struct! {
2020-02-03T21:30:08.6790092Z 35 | |       pub(crate) struct AnswerIndex {
2020-02-03T21:30:08.6790592Z 37 | |       }
2020-02-03T21:30:08.6790839Z 38 | |   }
2020-02-03T21:30:08.6791092Z    | |___- in this macro invocation
2020-02-03T21:30:08.6794351Z 
2020-02-03T21:30:08.6794351Z 
2020-02-03T21:30:08.6811239Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6811482Z   --> <::chalk_macros::index::index_struct macros>:22:74
2020-02-03T21:30:08.6811663Z    |
2020-02-03T21:30:08.6811939Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6812164Z 2  |   | {
2020-02-03T21:30:08.6812436Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6812817Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6813042Z ...    |
2020-02-03T21:30:08.6813337Z 22 |   |         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.6813723Z    |  _|__________________________________________________________________________^
2020-02-03T21:30:08.6814010Z 23 | | |         add_one (& self) -> Self
2020-02-03T21:30:08.6814307Z 24 | | |         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.6814641Z    | |_|_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6814837Z ...    |
2020-02-03T21:30:08.6815110Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6815597Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6815805Z    | 
2020-02-03T21:30:08.6816065Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2020-02-03T21:30:08.6816244Z    |
2020-02-03T21:30:08.6816244Z    |
2020-02-03T21:30:08.6816496Z 34 | /   index_struct! {
2020-02-03T21:30:08.6816759Z 35 | |       pub(crate) struct AnswerIndex {
2020-02-03T21:30:08.6817271Z 37 | |       }
2020-02-03T21:30:08.6817499Z 38 | |   }
2020-02-03T21:30:08.6817742Z    | |___- in this macro invocation
2020-02-03T21:30:08.6817789Z 
2020-02-03T21:30:08.6817789Z 
2020-02-03T21:30:08.6818017Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6818239Z   --> <::chalk_macros::index::index_struct macros>:24:65
2020-02-03T21:30:08.6818438Z    |
2020-02-03T21:30:08.6818726Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6818964Z 2  |   | {
2020-02-03T21:30:08.6819273Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6819610Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6819990Z ...    |
2020-02-03T21:30:08.6820290Z 24 |   |         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.6820710Z    |  _|_________________________________________________________________^
2020-02-03T21:30:08.6820986Z 25 | | |         (& self) -> Self
2020-02-03T21:30:08.6821293Z 26 | | |         { Self { value : usize :: sub_one (& self . value), } } fn add_usize
2020-02-03T21:30:08.6821654Z    | |_|_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6821863Z ...    |
2020-02-03T21:30:08.6822176Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6822694Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6822876Z    | 
2020-02-03T21:30:08.6823130Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2020-02-03T21:30:08.6823325Z    |
2020-02-03T21:30:08.6823325Z    |
2020-02-03T21:30:08.6823562Z 34 | /   index_struct! {
2020-02-03T21:30:08.6823820Z 35 | |       pub(crate) struct AnswerIndex {
2020-02-03T21:30:08.6824333Z 37 | |       }
2020-02-03T21:30:08.6824561Z 38 | |   }
2020-02-03T21:30:08.6824821Z    | |___- in this macro invocation
2020-02-03T21:30:08.6824852Z 
2020-02-03T21:30:08.6824852Z 
2020-02-03T21:30:08.6944782Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6945053Z   --> <::chalk_macros::index::index_struct macros>:26:65
2020-02-03T21:30:08.6945221Z    |
2020-02-03T21:30:08.6945481Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6945734Z 2  |   | {
2020-02-03T21:30:08.6946010Z 3  |   |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6946414Z 4  |   |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6946645Z ...    |
2020-02-03T21:30:08.6946919Z 26 |   |         { Self { value : usize :: sub_one (& self . value), } } fn add_usize
2020-02-03T21:30:08.6947211Z    |  _|_________________________________________________________________^
2020-02-03T21:30:08.6947619Z 27 | | |         (& self, n : usize) -> Option < Self >
2020-02-03T21:30:08.6947881Z 28 | | |         {
2020-02-03T21:30:08.6948160Z 29 | | |             usize :: add_usize (& self . value, n) . map
2020-02-03T21:30:08.6948447Z 30 | | |             (| value | Self { value })
2020-02-03T21:30:08.6948966Z    | |_|_________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6948966Z    | |_|_________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6949228Z 32 |   |     } impl From < usize > for $ n
2020-02-03T21:30:08.6949500Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6949979Z    |   |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6950143Z    | 
2020-02-03T21:30:08.6950405Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2020-02-03T21:30:08.6950568Z    |
2020-02-03T21:30:08.6950568Z    |
2020-02-03T21:30:08.6950784Z 34 | /   index_struct! {
2020-02-03T21:30:08.6951039Z 35 | |       pub(crate) struct AnswerIndex {
2020-02-03T21:30:08.6951489Z 37 | |       }
2020-02-03T21:30:08.6951720Z 38 | |   }
2020-02-03T21:30:08.6951940Z    | |___- in this macro invocation
2020-02-03T21:30:08.6954846Z 
2020-02-03T21:30:08.6954846Z 
2020-02-03T21:30:08.6960166Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6960417Z   --> <::chalk_macros::index::index_struct macros>:18:69
2020-02-03T21:30:08.6960612Z    |
2020-02-03T21:30:08.6960891Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6961235Z 2  |  | {
2020-02-03T21:30:08.6961583Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6961846Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6962129Z ...   |
2020-02-03T21:30:08.6962427Z 18 |  |         { usize :: steps_between (& start . value, & end . value) } fn
2020-02-03T21:30:08.6962872Z    |  |_____________________________________________________________________^
2020-02-03T21:30:08.6963150Z 19 | ||         replace_one (& mut self) -> Self
2020-02-03T21:30:08.6963437Z 20 | ||         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.6963765Z    | ||_______________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6963977Z ...   |
2020-02-03T21:30:08.6964239Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6964731Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6964911Z    | 
2020-02-03T21:30:08.6965148Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2020-02-03T21:30:08.6965320Z    |
2020-02-03T21:30:08.6965320Z    |
2020-02-03T21:30:08.6965551Z 81 | /  index_struct! {
2020-02-03T21:30:08.6965802Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2020-02-03T21:30:08.6966272Z 84 | |      }
2020-02-03T21:30:08.6966483Z 85 | |  }
2020-02-03T21:30:08.6966705Z    | |__- in this macro invocation
2020-02-03T21:30:08.6980385Z 
2020-02-03T21:30:08.6980385Z 
2020-02-03T21:30:08.6980794Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6981015Z   --> <::chalk_macros::index::index_struct macros>:20:73
2020-02-03T21:30:08.6981204Z    |
2020-02-03T21:30:08.6981465Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.6981974Z 2  |  | {
2020-02-03T21:30:08.6982324Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.6982590Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.6982900Z ...   |
2020-02-03T21:30:08.6983188Z 20 |  |         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.6983480Z    |  |_________________________________________________________________________^
2020-02-03T21:30:08.6983770Z 21 | ||         replace_zero (& mut self) -> Self
2020-02-03T21:30:08.6984068Z 22 | ||         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.6984426Z    | ||________________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.6984640Z ...   |
2020-02-03T21:30:08.6985101Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.6985588Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.6985759Z    | 
2020-02-03T21:30:08.6985989Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2020-02-03T21:30:08.6986181Z    |
2020-02-03T21:30:08.6986181Z    |
2020-02-03T21:30:08.6986398Z 81 | /  index_struct! {
2020-02-03T21:30:08.6986644Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2020-02-03T21:30:08.6987115Z 84 | |      }
2020-02-03T21:30:08.6987325Z 85 | |  }
2020-02-03T21:30:08.6987564Z    | |__- in this macro invocation
2020-02-03T21:30:08.6987593Z 
2020-02-03T21:30:08.6987593Z 
2020-02-03T21:30:08.7104495Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7104776Z   --> <::chalk_macros::index::index_struct macros>:22:74
2020-02-03T21:30:08.7104941Z    |
2020-02-03T21:30:08.7105202Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.7105440Z 2  |  | {
2020-02-03T21:30:08.7105839Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.7106155Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.7106436Z ...   |
2020-02-03T21:30:08.7106724Z 22 |  |         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.7106990Z    |  |__________________________________________________________________________^
2020-02-03T21:30:08.7107245Z 23 | ||         add_one (& self) -> Self
2020-02-03T21:30:08.7107554Z 24 | ||         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.7107870Z    | ||_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7108093Z ...   |
2020-02-03T21:30:08.7108357Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.7108851Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.7109017Z    | 
2020-02-03T21:30:08.7109267Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2020-02-03T21:30:08.7109431Z    |
2020-02-03T21:30:08.7109431Z    |
2020-02-03T21:30:08.7109648Z 81 | /  index_struct! {
2020-02-03T21:30:08.7109924Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2020-02-03T21:30:08.7110369Z 84 | |      }
2020-02-03T21:30:08.7110607Z 85 | |  }
2020-02-03T21:30:08.7110832Z    | |__- in this macro invocation
2020-02-03T21:30:08.7113654Z 
2020-02-03T21:30:08.7113654Z 
2020-02-03T21:30:08.7118718Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7118968Z   --> <::chalk_macros::index::index_struct macros>:24:65
2020-02-03T21:30:08.7119153Z    |
2020-02-03T21:30:08.7119413Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.7119633Z 2  |  | {
2020-02-03T21:30:08.7120027Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.7120315Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.7120522Z ...   |
2020-02-03T21:30:08.7120796Z 24 |  |         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.7121405Z 25 | ||         (& self) -> Self
2020-02-03T21:30:08.7121405Z 25 | ||         (& self) -> Self
2020-02-03T21:30:08.7121692Z 26 | ||         { Self { value : usize :: sub_one (& self . value), } } fn add_usize
2020-02-03T21:30:08.7122026Z    | ||_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7122227Z ...   |
2020-02-03T21:30:08.7122503Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.7122971Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.7123153Z    | 
2020-02-03T21:30:08.7123391Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2020-02-03T21:30:08.7123574Z    |
2020-02-03T21:30:08.7123574Z    |
2020-02-03T21:30:08.7123795Z 81 | /  index_struct! {
2020-02-03T21:30:08.7124051Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2020-02-03T21:30:08.7124516Z 84 | |      }
2020-02-03T21:30:08.7124726Z 85 | |  }
2020-02-03T21:30:08.7124971Z    | |__- in this macro invocation
2020-02-03T21:30:08.7128154Z 
2020-02-03T21:30:08.7128154Z 
2020-02-03T21:30:08.7136836Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7137106Z   --> <::chalk_macros::index::index_struct macros>:26:65
2020-02-03T21:30:08.7137287Z    |
2020-02-03T21:30:08.7137546Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.7137787Z 2  |  | {
2020-02-03T21:30:08.7138059Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.7138442Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.7138674Z ...   |
2020-02-03T21:30:08.7138948Z 26 |  |         { Self { value : usize :: sub_one (& self . value), } } fn add_usize
2020-02-03T21:30:08.7139313Z    |  |_________________________________________________________________^
2020-02-03T21:30:08.7139579Z 27 | ||         (& self, n : usize) -> Option < Self >
2020-02-03T21:30:08.7139960Z 28 | ||         {
2020-02-03T21:30:08.7140248Z 29 | ||             usize :: add_usize (& self . value, n) . map
2020-02-03T21:30:08.7140526Z 30 | ||             (| value | Self { value })
2020-02-03T21:30:08.7140765Z 31 | ||         }
2020-02-03T21:30:08.7141046Z    | ||_________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7141310Z 32 |  |     } impl From < usize > for $ n
2020-02-03T21:30:08.7141583Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.7142063Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.7142237Z    | 
2020-02-03T21:30:08.7142482Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2020-02-03T21:30:08.7142650Z    |
2020-02-03T21:30:08.7142650Z    |
2020-02-03T21:30:08.7142868Z 81 | /  index_struct! {
2020-02-03T21:30:08.7143130Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2020-02-03T21:30:08.7143586Z 84 | |      }
2020-02-03T21:30:08.7143818Z 85 | |  }
2020-02-03T21:30:08.7144052Z    | |__- in this macro invocation
2020-02-03T21:30:08.7144081Z 
2020-02-03T21:30:08.7144081Z 
2020-02-03T21:30:08.7307256Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7307534Z   --> <::chalk_macros::index::index_struct macros>:18:69
2020-02-03T21:30:08.7307725Z    |
2020-02-03T21:30:08.7308174Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.7308669Z 2  |  | {
2020-02-03T21:30:08.7309195Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.7309857Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.7310080Z ...   |
2020-02-03T21:30:08.7310412Z 18 |  |         { usize :: steps_between (& start . value, & end . value) } fn
2020-02-03T21:30:08.7310731Z    |  |_____________________________________________________________________^
2020-02-03T21:30:08.7311202Z 19 | ||         replace_one (& mut self) -> Self
2020-02-03T21:30:08.7311728Z 20 | ||         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.7312119Z    | ||_______________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7312548Z ...   |
2020-02-03T21:30:08.7313095Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.7314378Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.7314758Z    | 
2020-02-03T21:30:08.7315219Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2020-02-03T21:30:08.7317015Z    |
2020-02-03T21:30:08.7317015Z    |
2020-02-03T21:30:08.7317284Z 91 | /  index_struct! {
2020-02-03T21:30:08.7317832Z 93 | |          value: usize,
2020-02-03T21:30:08.7318423Z 94 | |      }
2020-02-03T21:30:08.7318862Z 95 | |  }
2020-02-03T21:30:08.7319281Z    | |__- in this macro invocation
2020-02-03T21:30:08.7319281Z    | |__- in this macro invocation
2020-02-03T21:30:08.7324050Z 
2020-02-03T21:30:08.7330284Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7330541Z   --> <::chalk_macros::index::index_struct macros>:20:73
2020-02-03T21:30:08.7330741Z    |
2020-02-03T21:30:08.7331028Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.7331716Z 2  |  | {
2020-02-03T21:30:08.7332249Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.7332536Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.7332875Z ...   |
2020-02-03T21:30:08.7333183Z 20 |  |         { Self { value : usize :: replace_one (& mut self . value), } } fn
2020-02-03T21:30:08.7333478Z    |  |_________________________________________________________________________^
2020-02-03T21:30:08.7333800Z 21 | ||         replace_zero (& mut self) -> Self
2020-02-03T21:30:08.7334118Z 22 | ||         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.7334497Z    | ||________________________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7334722Z ...   |
2020-02-03T21:30:08.7335181Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.7335695Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.7335898Z    | 
2020-02-03T21:30:08.7336148Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2020-02-03T21:30:08.7336338Z    |
2020-02-03T21:30:08.7336338Z    |
2020-02-03T21:30:08.7336594Z 91 | /  index_struct! {
2020-02-03T21:30:08.7337101Z 93 | |          value: usize,
2020-02-03T21:30:08.7337342Z 94 | |      }
2020-02-03T21:30:08.7337578Z 95 | |  }
2020-02-03T21:30:08.7337840Z    | |__- in this macro invocation
2020-02-03T21:30:08.7337840Z    | |__- in this macro invocation
2020-02-03T21:30:08.7349058Z 
2020-02-03T21:30:08.7349831Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7350296Z   --> <::chalk_macros::index::index_struct macros>:22:74
2020-02-03T21:30:08.7350490Z    |
2020-02-03T21:30:08.7350789Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
2020-02-03T21:30:08.7351061Z 2  |  | {
2020-02-03T21:30:08.7351532Z 3  |  |     # [derive (Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] $ v struct
2020-02-03T21:30:08.7351881Z 4  |  |     $ n { $ vf value : usize, } impl $ n
2020-02-03T21:30:08.7352102Z ...   |
2020-02-03T21:30:08.7352666Z 22 |  |         { Self { value : usize :: replace_zero (& mut self . value), } } fn
2020-02-03T21:30:08.7353159Z    |  |__________________________________________________________________________^
2020-02-03T21:30:08.7353451Z 23 | ||         add_one (& self) -> Self
2020-02-03T21:30:08.7353797Z 24 | ||         { Self { value : usize :: add_one (& self . value), } } fn sub_one
2020-02-03T21:30:08.7354321Z    | ||_______________________________________________________________^ not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7354564Z ...   |
2020-02-03T21:30:08.7355019Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2020-02-03T21:30:08.7355723Z    |  |_- in this expansion of `index_struct!`
2020-02-03T21:30:08.7355898Z    | 
2020-02-03T21:30:08.7356157Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2020-02-03T21:30:08.7356334Z    |
2020-02-03T21:30:08.7356334Z    |
2020-02-03T21:30:08.7356563Z 91 | /  index_struct! {
2020-02-03T21:30:08.7357070Z 93 | |          value: usize,
2020-02-03T21:30:08.7357296Z 94 | |      }
2020-02-03T21:30:08.7357531Z 95 | |  }
2020-02-03T21:30:08.7357777Z    | |__- in this macro invocation
2020-02-03T21:30:08.7357777Z    | |__- in this macro invocation
2020-02-03T21:30:08.7357807Z 
2020-02-03T21:30:08.7393758Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2020-02-03T21:30:08.7394014Z   --> <::chalk_macros::index::index_struct macros>:24:65
2020-02-03T21:30:08.7394211Z    |
2020-02-03T21:30:08.7394689Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize, }) =>
---
2020-02-03T21:30:08.8199635Z For more information about an error, try `rustc --explain E0200`.
2020-02-03T21:30:08.8233341Z error: could not compile `chalk-engine`.
2020-02-03T21:30:08.8251868Z warning: build failed, waiting for other jobs to finish...
2020-02-03T21:30:09.4114185Z error: build failed
2020-02-03T21:30:09.4114788Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-03T21:30:09.4114956Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-03T21:30:09.4115015Z Build completed unsuccessfully in 0:02:27
2020-02-03T21:30:09.4115051Z == clock drift check ==
2020-02-03T21:30:09.4115084Z   local time: Mon Feb  3 21:30:09 UTC 2020
2020-02-03T21:30:09.4115084Z   local time: Mon Feb  3 21:30:09 UTC 2020
2020-02-03T21:30:09.5415498Z   network time: Mon, 03 Feb 2020 21:30:09 GMT
2020-02-03T21:30:09.5415578Z == end clock drift check ==
2020-02-03T21:30:11.3177889Z 
2020-02-03T21:30:11.3262174Z ##[error]Bash exited with code '1'.
2020-02-03T21:30:11.3273069Z ##[section]Finishing: Run build
2020-02-03T21:30:11.3284324Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68807/merge to s
2020-02-03T21:30:11.3286281Z Task         : Get sources
2020-02-03T21:30:11.3286339Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-03T21:30:11.3286376Z Version      : 1.0.0
2020-02-03T21:30:11.3286409Z Author       : Microsoft
2020-02-03T21:30:11.3286409Z Author       : Microsoft
2020-02-03T21:30:11.3286462Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-03T21:30:11.3286502Z ==============================================================================
2020-02-03T21:30:11.6686161Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-03T21:30:11.6718086Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68807/merge to s
2020-02-03T21:30:11.6811141Z Cleaning up task key
2020-02-03T21:30:11.6811737Z Start cleaning up orphan processes.
2020-02-03T21:30:11.6934838Z Terminate orphan process: pid (4316) (python)
2020-02-03T21:30:11.7080203Z ##[section]Finishing: Finalize Job
