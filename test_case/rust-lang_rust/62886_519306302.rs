plain
2019-08-07T23:26:33.7810448Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-07T23:26:33.8009808Z ##[command]git config gc.auto 0
2019-08-07T23:26:33.8111281Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-07T23:26:33.8169017Z ##[command]git config --get-all http.proxy
2019-08-07T23:26:33.8325659Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-08-07T23:27:08.2031624Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T23:27:08.2031656Z 
2019-08-07T23:27:08.2031888Z   git checkout -b <new-branch-name>
2019-08-07T23:27:08.2031918Z 
2019-08-07T23:27:08.2031966Z HEAD is now at 4859960f7 Merge 2e95e253d72feda0aa4d122ffa47ad0923f9ecc1 into ad7c55e1fc55d9af4787b285cec1c64e3480ae84
2019-08-07T23:27:08.2188823Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T23:27:08.2191777Z ==============================================================================
2019-08-07T23:27:08.2191833Z Task         : Bash
2019-08-07T23:27:08.2191880Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T23:33:03.7496045Z     Checking lock_api v0.1.3
2019-08-07T23:33:04.0836183Z    Compiling rustc_version v0.2.3
2019-08-07T23:33:05.4106242Z     Checking polonius-engine v0.9.0
2019-08-07T23:33:06.0688989Z     Checking chalk-engine v0.9.0
2019-08-07T23:33:06.1565779Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1570674Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-08-07T23:33:06.1575168Z    |
2019-08-07T23:33:06.1579419Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1583973Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1587697Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1591538Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1595737Z ...    |
2019-08-07T23:33:06.1599564Z 13 |   | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-08-07T23:33:06.1603187Z    |   |______________________________________________________________^
2019-08-07T23:33:06.1608895Z 14 |  || & mut self ) -> Self {
2019-08-07T23:33:06.1615825Z 15 |  || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1622546Z    |  ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1629846Z ...    |
2019-08-07T23:33:06.1657746Z 23 |   | } impl From < usize > for $ n {
2019-08-07T23:33:06.1659425Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1660282Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1661634Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-07T23:33:06.1662232Z    |
2019-08-07T23:33:06.1662232Z    |
2019-08-07T23:33:06.1664428Z 15 | /   index_struct! {
2019-08-07T23:33:06.1665591Z 16 | |       pub(crate) struct StackIndex {
2019-08-07T23:33:06.1666819Z 17 | |           value: usize,
2019-08-07T23:33:06.1671036Z 19 | |   }
2019-08-07T23:33:06.1672236Z    | |___- in this macro invocation
2019-08-07T23:33:06.1672302Z 
2019-08-07T23:33:06.1672302Z 
2019-08-07T23:33:06.1672668Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1673060Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-08-07T23:33:06.1673286Z    |
2019-08-07T23:33:06.1673654Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1674463Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1674827Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1675987Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1676314Z ...    |
2019-08-07T23:33:06.1676663Z 15 |   | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1677125Z    |   |__________________________________________________________________^
2019-08-07T23:33:06.1677579Z 16 |  || replace_zero ( & mut self ) -> Self {
2019-08-07T23:33:06.1677972Z 17 |  || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1678423Z    |  ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1679389Z ...    |
2019-08-07T23:33:06.1679753Z 23 |   | } impl From < usize > for $ n {
2019-08-07T23:33:06.1680121Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1680633Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1681343Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-07T23:33:06.1681558Z    |
2019-08-07T23:33:06.1681558Z    |
2019-08-07T23:33:06.1681859Z 15 | /   index_struct! {
2019-08-07T23:33:06.1685407Z 16 | |       pub(crate) struct StackIndex {
2019-08-07T23:33:06.1685740Z 17 | |           value: usize,
2019-08-07T23:33:06.1686459Z 19 | |   }
2019-08-07T23:33:06.1686811Z    | |___- in this macro invocation
2019-08-07T23:33:06.1686872Z 
2019-08-07T23:33:06.1686872Z 
2019-08-07T23:33:06.1687304Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1687569Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-08-07T23:33:06.1687796Z    |
2019-08-07T23:33:06.1688127Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1688479Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1688824Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1689179Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1689433Z ...    |
2019-08-07T23:33:06.1698193Z 17 |   | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1698608Z    |  _|___________________________________________________________________^
2019-08-07T23:33:06.1699038Z 18 | | | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1699474Z    | |_|___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1699750Z ...    |
2019-08-07T23:33:06.1700065Z 23 |   | } impl From < usize > for $ n {
2019-08-07T23:33:06.1700439Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1700828Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1709890Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-07T23:33:06.1710282Z    |
2019-08-07T23:33:06.1710282Z    |
2019-08-07T23:33:06.1710602Z 15 |  /  index_struct! {
2019-08-07T23:33:06.1710911Z 16 |  |      pub(crate) struct StackIndex {
2019-08-07T23:33:06.1711207Z 17 |  |          value: usize,
2019-08-07T23:33:06.1711784Z 19 |  |  }
2019-08-07T23:33:06.1712257Z    |  |__- in this macro invocation
2019-08-07T23:33:06.1712311Z 
2019-08-07T23:33:06.1712311Z 
2019-08-07T23:33:06.1712618Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1712882Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-08-07T23:33:06.1713110Z    |
2019-08-07T23:33:06.1713442Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1713790Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1714137Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1714492Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1714748Z ...    |
2019-08-07T23:33:06.1715594Z 18 |   | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1716000Z    |  _|_____________________________________________________________________________^
2019-08-07T23:33:06.1716362Z 19 | | | sub_one ( & self ) -> Self {
2019-08-07T23:33:06.1716757Z 20 | | | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.1717181Z    | |_|________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1717436Z ...    |
2019-08-07T23:33:06.1717765Z 23 |   | } impl From < usize > for $ n {
2019-08-07T23:33:06.1718126Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1718519Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1719233Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-07T23:33:06.1719469Z    |
2019-08-07T23:33:06.1719469Z    |
2019-08-07T23:33:06.1719762Z 15 |  /  index_struct! {
2019-08-07T23:33:06.1720080Z 16 |  |      pub(crate) struct StackIndex {
2019-08-07T23:33:06.1720404Z 17 |  |          value: usize,
2019-08-07T23:33:06.1721121Z 19 |  |  }
2019-08-07T23:33:06.1721426Z    |  |__- in this macro invocation
2019-08-07T23:33:06.1721466Z 
2019-08-07T23:33:06.1721739Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1721739Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1722031Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-08-07T23:33:06.1722245Z    |
2019-08-07T23:33:06.1722587Z 1  |   / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1722977Z 2  |   | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1736306Z 3  |   | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1736885Z 4  |   | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1737149Z ...    |
2019-08-07T23:33:06.1737499Z 20 |   | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.1738669Z    |  _|__________________________________________________________^
2019-08-07T23:33:06.1792197Z 21 | | | & self , n : usize ) -> Option < Self > {
2019-08-07T23:33:06.1792867Z 22 | | | usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-08-07T23:33:06.1793336Z    | |_|______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1793673Z 23 |   | } impl From < usize > for $ n {
2019-08-07T23:33:06.1794347Z 24 |   | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1794998Z    |   |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1795561Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/stack.rs:15:1
2019-08-07T23:33:06.1795783Z    |
2019-08-07T23:33:06.1795783Z    |
2019-08-07T23:33:06.1796073Z 15 | /   index_struct! {
2019-08-07T23:33:06.1796399Z 16 | |       pub(crate) struct StackIndex {
2019-08-07T23:33:06.1796704Z 17 | |           value: usize,
2019-08-07T23:33:06.1797797Z 19 | |   }
2019-08-07T23:33:06.1798241Z    | |___- in this macro invocation
2019-08-07T23:33:06.1798282Z 
2019-08-07T23:33:06.1798282Z 
2019-08-07T23:33:06.1798594Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1798862Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-08-07T23:33:06.1799214Z    |
2019-08-07T23:33:06.1799548Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1799916Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1800249Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1800596Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1800844Z ...   |
2019-08-07T23:33:06.1801185Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-08-07T23:33:06.1801515Z    |  |______________________________________________________________^
2019-08-07T23:33:06.1801842Z 14 | || & mut self ) -> Self {
2019-08-07T23:33:06.1802191Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1802600Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1802843Z ...   |
2019-08-07T23:33:06.1803149Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1803874Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1804435Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1805046Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-07T23:33:06.1805283Z    |
2019-08-07T23:33:06.1805283Z    |
2019-08-07T23:33:06.1805606Z 34 | /  index_struct! {
2019-08-07T23:33:06.1805944Z 35 | |      pub(crate) struct AnswerIndex {
2019-08-07T23:33:06.1806279Z 36 | |          value: usize,
2019-08-07T23:33:06.1807032Z 38 | |  }
2019-08-07T23:33:06.1807367Z    | |__- in this macro invocation
2019-08-07T23:33:06.1807408Z 
2019-08-07T23:33:06.1807408Z 
2019-08-07T23:33:06.1807710Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1808007Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-08-07T23:33:06.1808255Z    |
2019-08-07T23:33:06.1808626Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1809190Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1809523Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1809868Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1810121Z ...   |
2019-08-07T23:33:06.1810454Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1810776Z    |  |__________________________________________________________________^
2019-08-07T23:33:06.1811125Z 16 | || replace_zero ( & mut self ) -> Self {
2019-08-07T23:33:06.1811487Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1811904Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1812146Z ...   |
2019-08-07T23:33:06.1812444Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1812790Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1813266Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1814372Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-07T23:33:06.1814598Z    |
2019-08-07T23:33:06.1814598Z    |
2019-08-07T23:33:06.1814894Z 34 | /  index_struct! {
2019-08-07T23:33:06.1815203Z 35 | |      pub(crate) struct AnswerIndex {
2019-08-07T23:33:06.1815609Z 36 | |          value: usize,
2019-08-07T23:33:06.1816233Z 38 | |  }
2019-08-07T23:33:06.1816527Z    | |__- in this macro invocation
2019-08-07T23:33:06.1816575Z 
2019-08-07T23:33:06.1816575Z 
2019-08-07T23:33:06.1816850Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1817121Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-08-07T23:33:06.1817344Z    |
2019-08-07T23:33:06.1817686Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1818061Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1818401Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1818911Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1819156Z ...   |
2019-08-07T23:33:06.1819499Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1819832Z    |  |___________________________________________________________________^
2019-08-07T23:33:06.1820212Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1820624Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1820878Z ...   |
2019-08-07T23:33:06.1821177Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1821510Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1822194Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1822780Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-07T23:33:06.1823020Z    |
2019-08-07T23:33:06.1823020Z    |
2019-08-07T23:33:06.1823333Z 34 | /  index_struct! {
2019-08-07T23:33:06.1823894Z 35 | |      pub(crate) struct AnswerIndex {
2019-08-07T23:33:06.1824205Z 36 | |          value: usize,
2019-08-07T23:33:06.1824921Z 38 | |  }
2019-08-07T23:33:06.1825218Z    | |__- in this macro invocation
2019-08-07T23:33:06.1825258Z 
2019-08-07T23:33:06.1825258Z 
2019-08-07T23:33:06.1825539Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1825812Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-08-07T23:33:06.1826029Z    |
2019-08-07T23:33:06.1826386Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1826759Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1827103Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1827469Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1827866Z ...   |
2019-08-07T23:33:06.1828215Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1828551Z    |  |_____________________________________________________________________________^
2019-08-07T23:33:06.1828883Z 19 | || sub_one ( & self ) -> Self {
2019-08-07T23:33:06.1829249Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.1829641Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1829891Z ...   |
2019-08-07T23:33:06.1830191Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1830531Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1831021Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1831569Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-07T23:33:06.1831802Z    |
2019-08-07T23:33:06.1831802Z    |
2019-08-07T23:33:06.1832273Z 34 | /  index_struct! {
2019-08-07T23:33:06.1832585Z 35 | |      pub(crate) struct AnswerIndex {
2019-08-07T23:33:06.1832892Z 36 | |          value: usize,
2019-08-07T23:33:06.1833773Z 38 | |  }
2019-08-07T23:33:06.1834118Z    | |__- in this macro invocation
2019-08-07T23:33:06.1834158Z 
2019-08-07T23:33:06.1834437Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1834437Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1834721Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-08-07T23:33:06.1834937Z    |
2019-08-07T23:33:06.1835280Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1835671Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1836005Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1836371Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1836619Z ...   |
2019-08-07T23:33:06.1836966Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.1837310Z    |  |__________________________________________________________^
2019-08-07T23:33:06.1837665Z 21 | || & self , n : usize ) -> Option < Self > {
2019-08-07T23:33:06.1838053Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-08-07T23:33:06.1838483Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1838808Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1839177Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1839685Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1840314Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/table.rs:34:1
2019-08-07T23:33:06.1840547Z    |
2019-08-07T23:33:06.1840547Z    |
2019-08-07T23:33:06.1840869Z 34 | /  index_struct! {
2019-08-07T23:33:06.1841207Z 35 | |      pub(crate) struct AnswerIndex {
2019-08-07T23:33:06.1841532Z 36 | |          value: usize,
2019-08-07T23:33:06.1842276Z 38 | |  }
2019-08-07T23:33:06.1896667Z    | |__- in this macro invocation
2019-08-07T23:33:06.1896738Z 
2019-08-07T23:33:06.1896738Z 
2019-08-07T23:33:06.1897222Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1897514Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-08-07T23:33:06.1897752Z    |
2019-08-07T23:33:06.1898098Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1899128Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1899480Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1899837Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1900103Z ...   |
2019-08-07T23:33:06.1900456Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-08-07T23:33:06.1900800Z    |  |______________________________________________________________^
2019-08-07T23:33:06.1901146Z 14 | || & mut self ) -> Self {
2019-08-07T23:33:06.1901509Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1901944Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1902199Z ...   |
2019-08-07T23:33:06.1902510Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1902883Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1903478Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1904335Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-07T23:33:06.1904553Z    |
2019-08-07T23:33:06.1904553Z    |
2019-08-07T23:33:06.1904861Z 81 | /  index_struct! {
2019-08-07T23:33:06.1905189Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-07T23:33:06.1905606Z 83 | |          value: usize,
2019-08-07T23:33:06.1906239Z 85 | |  }
2019-08-07T23:33:06.1906530Z    | |__- in this macro invocation
2019-08-07T23:33:06.1906586Z 
2019-08-07T23:33:06.1906586Z 
2019-08-07T23:33:06.1906868Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1907145Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-08-07T23:33:06.1907376Z    |
2019-08-07T23:33:06.1907721Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1908103Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1908455Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1908811Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1909074Z ...   |
2019-08-07T23:33:06.1909417Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1909760Z    |  |__________________________________________________________________^
2019-08-07T23:33:06.1910121Z 16 | || replace_zero ( & mut self ) -> Self {
2019-08-07T23:33:06.1910494Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1910927Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1911184Z ...   |
2019-08-07T23:33:06.1911587Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1912076Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1912454Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1913043Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-07T23:33:06.1913282Z    |
2019-08-07T23:33:06.1913282Z    |
2019-08-07T23:33:06.1914198Z 81 | /  index_struct! {
2019-08-07T23:33:06.1914776Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-07T23:33:06.1915204Z 83 | |          value: usize,
2019-08-07T23:33:06.1915830Z 85 | |  }
2019-08-07T23:33:06.1916124Z    | |__- in this macro invocation
2019-08-07T23:33:06.1916179Z 
2019-08-07T23:33:06.1916179Z 
2019-08-07T23:33:06.1916458Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1916731Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-08-07T23:33:06.1916961Z    |
2019-08-07T23:33:06.1917463Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1917811Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1918138Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1918470Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1918714Z ...   |
2019-08-07T23:33:06.1919045Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1919376Z    |  |___________________________________________________________________^
2019-08-07T23:33:06.1919739Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1920137Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1920387Z ...   |
2019-08-07T23:33:06.1920676Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1921005Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1921662Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1922212Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-07T23:33:06.1922456Z    |
2019-08-07T23:33:06.1922456Z    |
2019-08-07T23:33:06.1922760Z 81 | /  index_struct! {
2019-08-07T23:33:06.1923101Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-07T23:33:06.1924063Z 83 | |          value: usize,
2019-08-07T23:33:06.1924748Z 85 | |  }
2019-08-07T23:33:06.1925044Z    | |__- in this macro invocation
2019-08-07T23:33:06.1925084Z 
2019-08-07T23:33:06.1925084Z 
2019-08-07T23:33:06.1925359Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1925649Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-08-07T23:33:06.1925863Z    |
2019-08-07T23:33:06.1926209Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1926609Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1926943Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1927465Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1927702Z ...   |
2019-08-07T23:33:06.1928042Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1928401Z    |  |_____________________________________________________________________________^
2019-08-07T23:33:06.1928942Z 19 | || sub_one ( & self ) -> Self {
2019-08-07T23:33:06.1929294Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.1929673Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1929907Z ...   |
2019-08-07T23:33:06.1930211Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1930659Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1931030Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1931559Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-07T23:33:06.1931797Z    |
2019-08-07T23:33:06.1931797Z    |
2019-08-07T23:33:06.1932089Z 81 | /  index_struct! {
2019-08-07T23:33:06.1932415Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-07T23:33:06.1932872Z 83 | |          value: usize,
2019-08-07T23:33:06.1933487Z 85 | |  }
2019-08-07T23:33:06.1934503Z    | |__- in this macro invocation
2019-08-07T23:33:06.1934551Z 
2019-08-07T23:33:06.1934834Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1934834Z error[E0407]: method `add_usize` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1935130Z   --> <::chalk_macros::index::index_struct macros>:20:58
2019-08-07T23:33:06.1935347Z    |
2019-08-07T23:33:06.1935700Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1936090Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1936424Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1936795Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1937579Z ...   |
2019-08-07T23:33:06.1937929Z 20 |  | Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.1938275Z    |  |__________________________________________________________^
2019-08-07T23:33:06.1938612Z 21 | || & self , n : usize ) -> Option < Self > {
2019-08-07T23:33:06.1939048Z 22 | || usize :: add_usize ( & self . value , n ) . map ( | value | Self { value } ) }
2019-08-07T23:33:06.1939464Z    | ||______________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1939775Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1940296Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1940662Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1941224Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:81:1
2019-08-07T23:33:06.1941454Z    |
2019-08-07T23:33:06.1941454Z    |
2019-08-07T23:33:06.1941767Z 81 | /  index_struct! {
2019-08-07T23:33:06.1942105Z 82 | |      pub struct TableIndex { // FIXME: pub b/c Fold
2019-08-07T23:33:06.1942636Z 83 | |          value: usize,
2019-08-07T23:33:06.1943210Z 85 | |  }
2019-08-07T23:33:06.1943481Z    | |__- in this macro invocation
2019-08-07T23:33:06.1944014Z 
2019-08-07T23:33:06.1944014Z 
2019-08-07T23:33:06.1944330Z error[E0407]: method `replace_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1944604Z   --> <::chalk_macros::index::index_struct macros>:13:62
2019-08-07T23:33:06.1944836Z    |
2019-08-07T23:33:06.1945193Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1945566Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1945914Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1946271Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1946533Z ...   |
2019-08-07T23:33:06.1946885Z 13 |  | usize :: steps_between ( & start . value , & end . value ) } fn replace_one (
2019-08-07T23:33:06.1947367Z    |  |______________________________________________________________^
2019-08-07T23:33:06.1947686Z 14 | || & mut self ) -> Self {
2019-08-07T23:33:06.1948022Z 15 | || Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1948425Z    | ||________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1948665Z ...   |
2019-08-07T23:33:06.1948951Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1949429Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1949784Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1950320Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-07T23:33:06.1950538Z    |
2019-08-07T23:33:06.1950538Z    |
2019-08-07T23:33:06.1950828Z 91 | /  index_struct! {
2019-08-07T23:33:06.1951548Z 93 | |          value: usize,
2019-08-07T23:33:06.1951891Z 94 | |      }
2019-08-07T23:33:06.1952178Z 95 | |  }
2019-08-07T23:33:06.1952470Z    | |__- in this macro invocation
2019-08-07T23:33:06.1952470Z    | |__- in this macro invocation
2019-08-07T23:33:06.1952519Z 
2019-08-07T23:33:06.1952800Z error[E0407]: method `replace_zero` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1953069Z   --> <::chalk_macros::index::index_struct macros>:15:66
2019-08-07T23:33:06.1953295Z    |
2019-08-07T23:33:06.1954015Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1954453Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1954809Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1955168Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1955416Z ...   |
2019-08-07T23:33:06.1955775Z 15 |  | Self { value : usize :: replace_one ( & mut self . value ) , } } fn
2019-08-07T23:33:06.1956128Z    |  |__________________________________________________________________^
2019-08-07T23:33:06.1956478Z 16 | || replace_zero ( & mut self ) -> Self {
2019-08-07T23:33:06.1956850Z 17 | || Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1957421Z    | ||_________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1957743Z ...   |
2019-08-07T23:33:06.1958038Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1958511Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1958868Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1959408Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-07T23:33:06.1959798Z    |
2019-08-07T23:33:06.1959798Z    |
2019-08-07T23:33:06.1960074Z 91 | /  index_struct! {
2019-08-07T23:33:06.1960759Z 93 | |          value: usize,
2019-08-07T23:33:06.1961068Z 94 | |      }
2019-08-07T23:33:06.1961351Z 95 | |  }
2019-08-07T23:33:06.1961634Z    | |__- in this macro invocation
2019-08-07T23:33:06.1961634Z    | |__- in this macro invocation
2019-08-07T23:33:06.1961672Z 
2019-08-07T23:33:06.1961951Z error[E0407]: method `add_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1962213Z   --> <::chalk_macros::index::index_struct macros>:17:67
2019-08-07T23:33:06.1962422Z    |
2019-08-07T23:33:06.1962772Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1963130Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1963460Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1964169Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1964443Z ...   |
2019-08-07T23:33:06.1964809Z 17 |  | Self { value : usize :: replace_zero ( & mut self . value ) , } } fn add_one (
2019-08-07T23:33:06.1965165Z    |  |___________________________________________________________________^
2019-08-07T23:33:06.1965542Z 18 | || & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.1965984Z    | ||___________________________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1966237Z ...   |
2019-08-07T23:33:06.1966562Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.1966916Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.1967580Z    |  |________________________________________________________________- in this expansion of `index_struct!`
2019-08-07T23:33:06.1968147Z   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-engine-0.9.0/src/lib.rs:91:1
2019-08-07T23:33:06.1968385Z    |
2019-08-07T23:33:06.1968385Z    |
2019-08-07T23:33:06.1968829Z 91 | /  index_struct! {
2019-08-07T23:33:06.1969658Z 93 | |          value: usize,
2019-08-07T23:33:06.1969976Z 94 | |      }
2019-08-07T23:33:06.1970244Z 95 | |  }
2019-08-07T23:33:06.1970543Z    | |__- in this macro invocation
2019-08-07T23:33:06.1970543Z    | |__- in this macro invocation
2019-08-07T23:33:06.1970581Z 
2019-08-07T23:33:06.1970845Z error[E0407]: method `sub_one` is not a member of trait `std::iter::Step`
2019-08-07T23:33:06.1971121Z   --> <::chalk_macros::index::index_struct macros>:18:77
2019-08-07T23:33:06.1971329Z    |
2019-08-07T23:33:06.1971661Z 1  |  / ( $ v : vis struct $ n : ident { $ vf : vis value : usize , } ) => {
2019-08-07T23:33:06.1972039Z 2  |  | # [ derive ( Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash ) ] $ v
2019-08-07T23:33:06.1972361Z 3  |  | struct $ n { $ vf value : usize , } impl $ n {
2019-08-07T23:33:06.1972704Z 4  |  | # [ allow ( dead_code ) ] $ v fn get_and_increment ( & mut self ) -> Self {
2019-08-07T23:33:06.1972959Z ...   |
2019-08-07T23:33:06.1973447Z 18 |  | & self ) -> Self { Self { value : usize :: add_one ( & self . value ) , } } fn
2019-08-07T23:33:06.2014473Z    |  |_____________________________________________________________________________^
2019-08-07T23:33:06.2015050Z 19 | || sub_one ( & self ) -> Self {
2019-08-07T23:33:06.2015437Z 20 | || Self { value : usize :: sub_one ( & self . value ) , } } fn add_usize (
2019-08-07T23:33:06.2015873Z    | ||________________________________________________________^ not a member of trait `std::iter::Step`
2019-08-07T23:33:06.2016126Z ...   |
2019-08-07T23:33:06.2016452Z 23 |  | } impl From < usize > for $ n {
2019-08-07T23:33:06.2017203Z 24 |  | fn from ( value : usize ) -> Self { Self { value : value } } } }
2019-08-07T23:33:06.2017710Z    |  |________________________________________________________________- in this expansion of `index_struct!`
