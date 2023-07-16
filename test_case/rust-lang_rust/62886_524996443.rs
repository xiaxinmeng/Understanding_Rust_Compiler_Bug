plain
2019-08-26T19:19:46.3467565Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T19:19:46.3675172Z ##[command]git config gc.auto 0
2019-08-26T19:19:46.3761642Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T19:19:46.3809955Z ##[command]git config --get-all http.proxy
2019-08-26T19:19:46.3945760Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-08-26T19:20:21.2046241Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T19:20:21.2046435Z 
2019-08-26T19:20:21.2046807Z   git checkout -b <new-branch-name>
2019-08-26T19:20:21.2046970Z 
2019-08-26T19:20:21.2047143Z HEAD is now at 36143c39a Merge f651d33feb9f02bbe77174fd9ee579928b213b94 into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T19:20:21.2200770Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T19:20:21.2203632Z ==============================================================================
2019-08-26T19:20:21.2203684Z Task         : Bash
2019-08-26T19:20:21.2203727Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T19:26:18.6826706Z     Checking crossbeam-epoch v0.3.1
2019-08-26T19:26:19.0347072Z     Checking lock_api v0.1.3
2019-08-26T19:26:19.3152652Z     Checking polonius-engine v0.9.0
2019-08-26T19:26:20.0850315Z     Checking chalk-engine v0.9.0
2019-08-26T19:26:20.1673910Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1674471Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-26T19:26:20.1674716Z    |
2019-08-26T19:26:20.1675224Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.1675529Z 2  |   | {
2019-08-26T19:26:20.1675881Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.1676223Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.1676474Z ...    |
2019-08-26T19:26:20.1676823Z 18 |   |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-26T19:26:20.1677171Z    |  _|______________________________________________________________________^
2019-08-26T19:26:20.1677709Z 19 | | |         replace_one (& mut self) -> Self
2019-08-26T19:26:20.1678151Z 20 | | |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.1678561Z    | |_|________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1678964Z ...    |
2019-08-26T19:26:20.1679301Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.1679615Z 34 |   | }
2019-08-26T19:26:20.1679928Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.1680520Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-26T19:26:20.1680768Z    |
2019-08-26T19:26:20.1680768Z    |
2019-08-26T19:26:20.1681079Z 15 |  /  index_struct! {
2019-08-26T19:26:20.1681545Z 16 |  |      pub(crate) struct StackIndex {
2019-08-26T19:26:20.1682435Z 17 |  |          value: usize,
2019-08-26T19:26:20.1683108Z 19 |  |  }
2019-08-26T19:26:20.1683422Z    |  |__- in this macro invocation
2019-08-26T19:26:20.1683461Z 
2019-08-26T19:26:20.1683461Z 
2019-08-26T19:26:20.1684093Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1686436Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-26T19:26:20.1686704Z    |
2019-08-26T19:26:20.1689508Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.1689867Z 2  |   | {
2019-08-26T19:26:20.1690246Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.1690568Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.1690839Z ...    |
2019-08-26T19:26:20.1691176Z 20 |   |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.1692300Z    |  _|__________________________________________________________________________^
2019-08-26T19:26:20.1692788Z 21 | | |         replace_zero (& mut self) -> Self
2019-08-26T19:26:20.1693182Z 22 | | |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.1693634Z    | |_|_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1694037Z ...    |
2019-08-26T19:26:20.1694414Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.1694720Z 34 |   | }
2019-08-26T19:26:20.1695190Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.1695827Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-26T19:26:20.1696065Z    |
2019-08-26T19:26:20.1696065Z    |
2019-08-26T19:26:20.1696334Z 15 | /   index_struct! {
2019-08-26T19:26:20.1696625Z 16 | |       pub(crate) struct StackIndex {
2019-08-26T19:26:20.1696932Z 17 | |           value: usize,
2019-08-26T19:26:20.1697461Z 19 | |   }
2019-08-26T19:26:20.1697753Z    | |___- in this macro invocation
2019-08-26T19:26:20.1697790Z 
2019-08-26T19:26:20.1789744Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1789744Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1797187Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-26T19:26:20.1797545Z    |
2019-08-26T19:26:20.1797917Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.1798237Z 2  |   | {
2019-08-26T19:26:20.1798642Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.1798987Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.1799270Z ...    |
2019-08-26T19:26:20.1799631Z 22 |   |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.1800239Z    |  _|___________________________________________________________________________^
2019-08-26T19:26:20.1800725Z 23 | | |         add_one (& self) -> Self
2019-08-26T19:26:20.1801134Z 24 | | |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.1801668Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1802677Z ...    |
2019-08-26T19:26:20.1803052Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.1803374Z 34 |   | }
2019-08-26T19:26:20.1803702Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.1804316Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-26T19:26:20.1804542Z    |
2019-08-26T19:26:20.1804542Z    |
2019-08-26T19:26:20.1804870Z 15 | /   index_struct! {
2019-08-26T19:26:20.1805187Z 16 | |       pub(crate) struct StackIndex {
2019-08-26T19:26:20.1805789Z 17 | |           value: usize,
2019-08-26T19:26:20.1806354Z 19 | |   }
2019-08-26T19:26:20.1806646Z    | |___- in this macro invocation
2019-08-26T19:26:20.1806723Z 
2019-08-26T19:26:20.1806723Z 
2019-08-26T19:26:20.1869087Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1869435Z   --> <::chalk_macros::index::index_struct macros>:24:66
2019-08-26T19:26:20.1870082Z    |
2019-08-26T19:26:20.1870430Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.1870723Z 2  |   | {
2019-08-26T19:26:20.1871100Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.1871540Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.1872442Z ...    |
2019-08-26T19:26:20.1872892Z 24 |   |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.1873262Z    |  _|__________________________________________________________________^
2019-08-26T19:26:20.1873785Z 25 | | |         (& self) -> Self
2019-08-26T19:26:20.1874218Z 26 | | |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-26T19:26:20.1874656Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1875047Z ...    |
2019-08-26T19:26:20.1875524Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.1875852Z 34 |   | }
2019-08-26T19:26:20.1876174Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.1876714Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-26T19:26:20.1876930Z    |
2019-08-26T19:26:20.1876930Z    |
2019-08-26T19:26:20.1877348Z 15 | /   index_struct! {
2019-08-26T19:26:20.1877654Z 16 | |       pub(crate) struct StackIndex {
2019-08-26T19:26:20.1877939Z 17 | |           value: usize,
2019-08-26T19:26:20.1878502Z 19 | |   }
2019-08-26T19:26:20.1878795Z    | |___- in this macro invocation
2019-08-26T19:26:20.1879475Z 
2019-08-26T19:26:20.1879838Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1879838Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1880137Z   --> <::chalk_macros::index::index_struct macros>:26:66
2019-08-26T19:26:20.1880347Z    |
2019-08-26T19:26:20.1880670Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.1880967Z 2  |   | {
2019-08-26T19:26:20.1881312Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.1882207Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.1882546Z ...    |
2019-08-26T19:26:20.1882917Z 26 |   |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-26T19:26:20.1883320Z    |  _|__________________________________________________________________^
2019-08-26T19:26:20.1883807Z 27 | | |         (& self , n : usize) -> Option < Self >
2019-08-26T19:26:20.1884208Z 28 | | |         {
2019-08-26T19:26:20.1884577Z 29 | | |             usize :: add_usize (& self . value , n) . map
2019-08-26T19:26:20.1884949Z 30 | | |             (| value | Self { value })
2019-08-26T19:26:20.1885991Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1885991Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.1886325Z 32 |   |     } impl From < usize > for $ n
2019-08-26T19:26:20.1886659Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.1886957Z 34 |   | }
2019-08-26T19:26:20.1887274Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.1887861Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-26T19:26:20.1888092Z    |
2019-08-26T19:26:20.1888092Z    |
2019-08-26T19:26:20.1888413Z 15 | /   index_struct! {
2019-08-26T19:26:20.1888738Z 16 | |       pub(crate) struct StackIndex {
2019-08-26T19:26:20.1889052Z 17 | |           value: usize,
2019-08-26T19:26:20.1889665Z 19 | |   }
2019-08-26T19:26:20.1889969Z    | |___- in this macro invocation
2019-08-26T19:26:20.1890024Z 
2019-08-26T19:26:20.1890024Z 
2019-08-26T19:26:20.2073425Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2073778Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-26T19:26:20.2074037Z    |
2019-08-26T19:26:20.2074395Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2074699Z 2  |   | {
2019-08-26T19:26:20.2075099Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2075553Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2075805Z ...    |
2019-08-26T19:26:20.2076303Z 18 |   |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-26T19:26:20.2076704Z    |  _|______________________________________________________________________^
2019-08-26T19:26:20.2077060Z 19 | | |         replace_one (& mut self) -> Self
2019-08-26T19:26:20.2077541Z 20 | | |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.2077972Z    | |_|________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2078223Z ...    |
2019-08-26T19:26:20.2078556Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2078857Z 34 |   | }
2019-08-26T19:26:20.2079175Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2079757Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-26T19:26:20.2079986Z    |
2019-08-26T19:26:20.2079986Z    |
2019-08-26T19:26:20.2080314Z 34 | /   index_struct! {
2019-08-26T19:26:20.2080636Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-26T19:26:20.2080947Z 36 | |           value: usize,
2019-08-26T19:26:20.2081673Z 38 | |   }
2019-08-26T19:26:20.2082488Z    | |___- in this macro invocation
2019-08-26T19:26:20.2082537Z 
2019-08-26T19:26:20.2082537Z 
2019-08-26T19:26:20.2082835Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2083132Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-26T19:26:20.2083377Z    |
2019-08-26T19:26:20.2083726Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2084024Z 2  |   | {
2019-08-26T19:26:20.2084407Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2084765Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2085041Z ...    |
2019-08-26T19:26:20.2085621Z 20 |   |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.2086032Z    |  _|__________________________________________________________________________^
2019-08-26T19:26:20.2086372Z 21 | | |         replace_zero (& mut self) -> Self
2019-08-26T19:26:20.2086861Z 22 | | |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.2087281Z    | |_|_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2087546Z ...    |
2019-08-26T19:26:20.2087878Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2088154Z 34 |   | }
2019-08-26T19:26:20.2088474Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2089087Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-26T19:26:20.2089335Z    |
2019-08-26T19:26:20.2089335Z    |
2019-08-26T19:26:20.2089641Z 34 | /   index_struct! {
2019-08-26T19:26:20.2089968Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-26T19:26:20.2090297Z 36 | |           value: usize,
2019-08-26T19:26:20.2090916Z 38 | |   }
2019-08-26T19:26:20.2091225Z    | |___- in this macro invocation
2019-08-26T19:26:20.2091262Z 
2019-08-26T19:26:20.2091664Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2091664Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2092469Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-26T19:26:20.2092713Z    |
2019-08-26T19:26:20.2093059Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2093381Z 2  |   | {
2019-08-26T19:26:20.2093747Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2094117Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2094376Z ...    |
2019-08-26T19:26:20.2094847Z 22 |   |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.2095406Z    |  _|___________________________________________________________________________^
2019-08-26T19:26:20.2095862Z 23 | | |         add_one (& self) -> Self
2019-08-26T19:26:20.2096339Z 24 | | |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.2096751Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2096996Z ...    |
2019-08-26T19:26:20.2097340Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2097738Z 34 |   | }
2019-08-26T19:26:20.2098063Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2098639Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-26T19:26:20.2098887Z    |
2019-08-26T19:26:20.2098887Z    |
2019-08-26T19:26:20.2099205Z 34 | /   index_struct! {
2019-08-26T19:26:20.2099541Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-26T19:26:20.2099880Z 36 | |           value: usize,
2019-08-26T19:26:20.2100491Z 38 | |   }
2019-08-26T19:26:20.2100946Z    | |___- in this macro invocation
2019-08-26T19:26:20.2100983Z 
2019-08-26T19:26:20.2100983Z 
2019-08-26T19:26:20.2189484Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2189835Z   --> <::chalk_macros::index::index_struct macros>:24:66
2019-08-26T19:26:20.2190056Z    |
2019-08-26T19:26:20.2190768Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2191079Z 2  |   | {
2019-08-26T19:26:20.2191421Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2192488Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2192811Z ...    |
2019-08-26T19:26:20.2193340Z 24 |   |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.2193786Z    |  _|__________________________________________________________________^
2019-08-26T19:26:20.2194129Z 25 | | |         (& self) -> Self
2019-08-26T19:26:20.2194532Z 26 | | |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-26T19:26:20.2195079Z    | |_|________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2195463Z ...    |
2019-08-26T19:26:20.2195939Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2196213Z 34 |   | }
2019-08-26T19:26:20.2196513Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2197043Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-26T19:26:20.2197266Z    |
2019-08-26T19:26:20.2197266Z    |
2019-08-26T19:26:20.2197540Z 34 | /   index_struct! {
2019-08-26T19:26:20.2197841Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-26T19:26:20.2198142Z 36 | |           value: usize,
2019-08-26T19:26:20.2198676Z 38 | |   }
2019-08-26T19:26:20.2198983Z    | |___- in this macro invocation
2019-08-26T19:26:20.2199660Z 
2019-08-26T19:26:20.2200019Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2200019Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2200305Z   --> <::chalk_macros::index::index_struct macros>:26:66
2019-08-26T19:26:20.2200522Z    |
2019-08-26T19:26:20.2200847Z 1  |   / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2201142Z 2  |   | {
2019-08-26T19:26:20.2201606Z 3  |   |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2202781Z 4  |   |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2203090Z ...    |
2019-08-26T19:26:20.2203581Z 26 |   |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-26T19:26:20.2204015Z    |  _|__________________________________________________________________^
2019-08-26T19:26:20.2204378Z 27 | | |         (& self , n : usize) -> Option < Self >
2019-08-26T19:26:20.2204700Z 28 | | |         {
2019-08-26T19:26:20.2205319Z 29 | | |             usize :: add_usize (& self . value , n) . map
2019-08-26T19:26:20.2205773Z 30 | | |             (| value | Self { value })
2019-08-26T19:26:20.2206451Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2206451Z    | |_|_________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2206754Z 32 |   |     } impl From < usize > for $ n
2019-08-26T19:26:20.2207114Z 33 |   |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2207393Z 34 |   | }
2019-08-26T19:26:20.2207712Z    |   |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2208272Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-26T19:26:20.2208518Z    |
2019-08-26T19:26:20.2208518Z    |
2019-08-26T19:26:20.2208815Z 34 | /   index_struct! {
2019-08-26T19:26:20.2209135Z 35 | |       pub(crate) struct AnswerIndex {
2019-08-26T19:26:20.2209473Z 36 | |           value: usize,
2019-08-26T19:26:20.2210070Z 38 | |   }
2019-08-26T19:26:20.2210393Z    | |___- in this macro invocation
2019-08-26T19:26:20.2210431Z 
2019-08-26T19:26:20.2210431Z 
2019-08-26T19:26:20.2350229Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2350561Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-26T19:26:20.2350777Z    |
2019-08-26T19:26:20.2351660Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2352598Z 2  |  | {
2019-08-26T19:26:20.2353152Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2353578Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2353827Z ...   |
2019-08-26T19:26:20.2354182Z 18 |  |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-26T19:26:20.2354672Z    |  |______________________________________________________________________^
2019-08-26T19:26:20.2355016Z 19 | ||         replace_one (& mut self) -> Self
2019-08-26T19:26:20.2355522Z 20 | ||         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.2355930Z    | ||________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2356169Z ...   |
2019-08-26T19:26:20.2356521Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2356811Z 34 |  | }
2019-08-26T19:26:20.2357125Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2357633Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-26T19:26:20.2357855Z    |
2019-08-26T19:26:20.2357855Z    |
2019-08-26T19:26:20.2358128Z 81 | /  index_struct! {
2019-08-26T19:26:20.2358434Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-26T19:26:20.2358744Z 83 | |          value: usize,
2019-08-26T19:26:20.2359271Z 85 | |  }
2019-08-26T19:26:20.2359574Z    | |__- in this macro invocation
2019-08-26T19:26:20.2360379Z 
2019-08-26T19:26:20.2360379Z 
2019-08-26T19:26:20.2360741Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2361027Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-26T19:26:20.2361235Z    |
2019-08-26T19:26:20.2361698Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2362642Z 2  |  | {
2019-08-26T19:26:20.2363432Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2363930Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2364226Z ...   |
2019-08-26T19:26:20.2364585Z 20 |  |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.2364953Z    |  |__________________________________________________________________________^
2019-08-26T19:26:20.2365406Z 21 | ||         replace_zero (& mut self) -> Self
2019-08-26T19:26:20.2365909Z 22 | ||         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.2366316Z    | ||_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2366557Z ...   |
2019-08-26T19:26:20.2366898Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2367184Z 34 |  | }
2019-08-26T19:26:20.2367496Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2368054Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-26T19:26:20.2368297Z    |
2019-08-26T19:26:20.2368297Z    |
2019-08-26T19:26:20.2368595Z 81 | /  index_struct! {
2019-08-26T19:26:20.2368925Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-26T19:26:20.2369249Z 83 | |          value: usize,
2019-08-26T19:26:20.2369845Z 85 | |  }
2019-08-26T19:26:20.2370176Z    | |__- in this macro invocation
2019-08-26T19:26:20.2370215Z 
2019-08-26T19:26:20.2370498Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2370498Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2370791Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-26T19:26:20.2371410Z    |
2019-08-26T19:26:20.2372376Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2372761Z 2  |  | {
2019-08-26T19:26:20.2373127Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2373586Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2373906Z ...   |
2019-08-26T19:26:20.2374266Z 22 |  |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.2374630Z    |  |___________________________________________________________________________^
2019-08-26T19:26:20.2375074Z 23 | ||         add_one (& self) -> Self
2019-08-26T19:26:20.2375552Z 24 | ||         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.2375974Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2376217Z ...   |
2019-08-26T19:26:20.2376558Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2376847Z 34 |  | }
2019-08-26T19:26:20.2377144Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2377705Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-26T19:26:20.2377937Z    |
2019-08-26T19:26:20.2377937Z    |
2019-08-26T19:26:20.2378251Z 81 | /  index_struct! {
2019-08-26T19:26:20.2378584Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-26T19:26:20.2378893Z 83 | |          value: usize,
2019-08-26T19:26:20.2379504Z 85 | |  }
2019-08-26T19:26:20.2379820Z    | |__- in this macro invocation
2019-08-26T19:26:20.2380423Z 
2019-08-26T19:26:20.2380423Z 
2019-08-26T19:26:20.2494132Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2494496Z   --> <::chalk_macros::index::index_struct macros>:24:66
2019-08-26T19:26:20.2495282Z    |
2019-08-26T19:26:20.2495755Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2496058Z 2  |  | {
2019-08-26T19:26:20.2496415Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2496751Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2497131Z ...   |
2019-08-26T19:26:20.2497517Z 24 |  |         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.2497860Z    |  |__________________________________________________________________^
2019-08-26T19:26:20.2498170Z 25 | ||         (& self) -> Self
2019-08-26T19:26:20.2498655Z 26 | ||         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-26T19:26:20.2499060Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2499305Z ...   |
2019-08-26T19:26:20.2499654Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2499932Z 34 |  | }
2019-08-26T19:26:20.2500259Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2500813Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-26T19:26:20.2501060Z    |
2019-08-26T19:26:20.2501060Z    |
2019-08-26T19:26:20.2501367Z 81 | /  index_struct! {
2019-08-26T19:26:20.2502290Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-26T19:26:20.2502679Z 83 | |          value: usize,
2019-08-26T19:26:20.2503282Z 85 | |  }
2019-08-26T19:26:20.2503767Z    | |__- in this macro invocation
2019-08-26T19:26:20.2504502Z 
2019-08-26T19:26:20.2504903Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2504903Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2505317Z   --> <::chalk_macros::index::index_struct macros>:26:66
2019-08-26T19:26:20.2505530Z    |
2019-08-26T19:26:20.2505867Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2506142Z 2  |  | {
2019-08-26T19:26:20.2506485Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2506823Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2507060Z ...   |
2019-08-26T19:26:20.2507535Z 26 |  |         { Self { value : usize :: sub_one (& self . value) , } } fn add_usize
2019-08-26T19:26:20.2507904Z    |  |__________________________________________________________________^
2019-08-26T19:26:20.2508250Z 27 | ||         (& self , n : usize) -> Option < Self >
2019-08-26T19:26:20.2508712Z 28 | ||         {
2019-08-26T19:26:20.2509065Z 29 | ||             usize :: add_usize (& self . value , n) . map
2019-08-26T19:26:20.2509401Z 30 | ||             (| value | Self { value })
2019-08-26T19:26:20.2509701Z 31 | ||         }
2019-08-26T19:26:20.2510054Z    | ||_________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2510356Z 32 |  |     } impl From < usize > for $ n
2019-08-26T19:26:20.2510710Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2510986Z 34 |  | }
2019-08-26T19:26:20.2511292Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2512378Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-26T19:26:20.2512614Z    |
2019-08-26T19:26:20.2512614Z    |
2019-08-26T19:26:20.2512926Z 81 | /  index_struct! {
2019-08-26T19:26:20.2513275Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-26T19:26:20.2513596Z 83 | |          value: usize,
2019-08-26T19:26:20.2514175Z 85 | |  }
2019-08-26T19:26:20.2514494Z    | |__- in this macro invocation
2019-08-26T19:26:20.2514533Z 
2019-08-26T19:26:20.2514533Z 
2019-08-26T19:26:20.2628854Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2629194Z   --> <::chalk_macros::index::index_struct macros>:18:70
2019-08-26T19:26:20.2629833Z    |
2019-08-26T19:26:20.2630192Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2630477Z 2  |  | {
2019-08-26T19:26:20.2630970Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2631371Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2631733Z ...   |
2019-08-26T19:26:20.2632961Z 18 |  |         { usize :: steps_between (& start . value , & end . value) } fn
2019-08-26T19:26:20.2633509Z    |  |______________________________________________________________________^
2019-08-26T19:26:20.2633854Z 19 | ||         replace_one (& mut self) -> Self
2019-08-26T19:26:20.2634265Z 20 | ||         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.2634695Z    | ||________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2634979Z ...   |
2019-08-26T19:26:20.2635338Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2635915Z 34 |  | }
2019-08-26T19:26:20.2636225Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2636741Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-26T19:26:20.2636950Z    |
2019-08-26T19:26:20.2636950Z    |
2019-08-26T19:26:20.2637220Z 91 | /  index_struct! {
2019-08-26T19:26:20.2637813Z 93 | |          value: usize,
2019-08-26T19:26:20.2638080Z 94 | |      }
2019-08-26T19:26:20.2638369Z 95 | |  }
2019-08-26T19:26:20.2638649Z    | |__- in this macro invocation
2019-08-26T19:26:20.2638649Z    | |__- in this macro invocation
2019-08-26T19:26:20.2639313Z 
2019-08-26T19:26:20.2640065Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2640362Z   --> <::chalk_macros::index::index_struct macros>:20:74
2019-08-26T19:26:20.2640568Z    |
2019-08-26T19:26:20.2640930Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2641207Z 2  |  | {
2019-08-26T19:26:20.2642229Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2642695Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2642951Z ...   |
2019-08-26T19:26:20.2643341Z 20 |  |         { Self { value : usize :: replace_one (& mut self . value) , } } fn
2019-08-26T19:26:20.2643827Z    |  |__________________________________________________________________________^
2019-08-26T19:26:20.2644197Z 21 | ||         replace_zero (& mut self) -> Self
2019-08-26T19:26:20.2644589Z 22 | ||         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.2645039Z    | ||_________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2645429Z ...   |
2019-08-26T19:26:20.2645897Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2646184Z 34 |  | }
2019-08-26T19:26:20.2646482Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2647001Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-26T19:26:20.2647205Z    |
2019-08-26T19:26:20.2647205Z    |
2019-08-26T19:26:20.2647492Z 91 | /  index_struct! {
2019-08-26T19:26:20.2648077Z 93 | |          value: usize,
2019-08-26T19:26:20.2648346Z 94 | |      }
2019-08-26T19:26:20.2648606Z 95 | |  }
2019-08-26T19:26:20.2648905Z    | |__- in this macro invocation
2019-08-26T19:26:20.2648905Z    | |__- in this macro invocation
2019-08-26T19:26:20.2648942Z 
2019-08-26T19:26:20.2649866Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2650154Z   --> <::chalk_macros::index::index_struct macros>:22:75
2019-08-26T19:26:20.2650363Z    |
2019-08-26T19:26:20.2650684Z 1  |  / ($ v : vis struct $ n : ident { $ vf : vis value : usize , }) =>
2019-08-26T19:26:20.2650986Z 2  |  | {
2019-08-26T19:26:20.2651323Z 3  |  |     # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] $ v
2019-08-26T19:26:20.2652300Z 4  |  |     struct $ n { $ vf value : usize , } impl $ n
2019-08-26T19:26:20.2652662Z ...   |
2019-08-26T19:26:20.2653018Z 22 |  |         { Self { value : usize :: replace_zero (& mut self . value) , } } fn
2019-08-26T19:26:20.2653386Z    |  |___________________________________________________________________________^
2019-08-26T19:26:20.2653832Z 23 | ||         add_one (& self) -> Self
2019-08-26T19:26:20.2654208Z 24 | ||         { Self { value : usize :: add_one (& self . value) , } } fn sub_one
2019-08-26T19:26:20.2654656Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-26T19:26:20.2654913Z ...   |
2019-08-26T19:26:20.2655426Z 33 |  |     { fn from (value : usize) -> Self { Self { value : value } } }
2019-08-26T19:26:20.2655724Z 34 |  | }
2019-08-26T19:26:20.2656026Z    |  |_- in this expansion of `index_struct!`
2019-08-26T19:26:20.2656545Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-26T19:26:20.2656751Z    |
2019-08-26T19:26:20.2656751Z    |
2019-08-26T19:26:20.2657039Z 91 | /  index_struct! {
2019-08-26T19:26:20.2657603Z 93 | |          value: usize,
2019-08-26T19:26:20.2657896Z 94 | |      }
2019-08-26T19:26:20.2658157Z 95 | |  }
2019-08-26T19:26:20.2658435Z    | |__- in this macro invocation
2019-08-26T19:26:20.2658435Z    | |__- in this macro invocation
2019-08-26T19:26:20.2658487Z 
2019-08-26T19:26:20.2788995Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
---
2019-08-26T19:26:21.8452992Z == clock drift check ==
2019-08-26T19:26:21.8474864Z   local time: Mon Aug 26 19:26:21 UTC 2019
2019-08-26T19:26:22.0027517Z   network time: Mon, 26 Aug 2019 19:26:21 GMT
2019-08-26T19:26:22.0028902Z == end clock drift check ==
2019-08-26T19:26:24.8457161Z ##[error]Bash exited with code '1'.
2019-08-26T19:26:24.8497575Z ##[section]Starting: Checkout
2019-08-26T19:26:24.8498994Z ==============================================================================
2019-08-26T19:26:24.8499054Z Task         : Get sources
2019-08-26T19:26:24.8499095Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
