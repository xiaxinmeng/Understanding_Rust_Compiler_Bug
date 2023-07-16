plain
2019-07-25T09:11:28.3854234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T09:11:28.4061458Z ##[command]git config gc.auto 0
2019-07-25T09:11:28.4130369Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T09:11:28.4179488Z ##[command]git config --get-all http.proxy
2019-07-25T09:11:28.4323198Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62848/merge:refs/remotes/pull/62848/merge
---
2019-07-25T09:12:02.6993086Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T09:12:02.6993117Z 
2019-07-25T09:12:02.6993324Z   git checkout -b <new-branch-name>
2019-07-25T09:12:02.6993557Z 
2019-07-25T09:12:02.6993607Z HEAD is now at 403eef78c Merge da49502a7b57739778055558264cd9d8e58661fd into 185b9acb66438894596f3c40d2ae4c6f7deeb8ab
2019-07-25T09:12:02.7127753Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T09:12:02.7130635Z ==============================================================================
2019-07-25T09:12:02.7130712Z Task         : Bash
2019-07-25T09:12:02.7130756Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T09:19:45.2277704Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-25T09:20:36.9711624Z     Checking rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
2019-07-25T09:20:38.4902210Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-25T09:20:39.7833599Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-25T09:20:40.5412550Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5441196Z   --> <::rustc_data_structures::indexed_vec::newtype_index macros>:14:64
2019-07-25T09:20:40.5441512Z    |
2019-07-25T09:20:40.5441811Z 14 | derive ( Copy , PartialEq , Eq , Hash , PartialOrd , Ord , $ ( $ derives ) , *
2019-07-25T09:20:40.5442621Z    |                                                                ^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5442674Z 
2019-07-25T09:20:40.5450111Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5460917Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.5466743Z     |
2019-07-25T09:20:40.5472549Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.5527401Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.5528061Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.5528372Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.5528619Z ...     |
2019-07-25T09:20:40.5529318Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.5529669Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.5530020Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.5530580Z ...     |
2019-07-25T09:20:40.5530580Z ...     |
2019-07-25T09:20:40.5530893Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.5531268Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.5531582Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.5531883Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.5532240Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5532473Z ...     |
2019-07-25T09:20:40.5532801Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.5533089Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.5559254Z     |   |                    |
2019-07-25T09:20:40.5559254Z     |   |                    |
2019-07-25T09:20:40.5559562Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.5559887Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.5560194Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.5560862Z    ::: src/librustc_mir/borrow_check/location.rs:20:1
2019-07-25T09:20:40.5561059Z     |
2019-07-25T09:20:40.5561341Z 20  | /   newtype_index! {
2019-07-25T09:20:40.5561621Z 21  | |       pub struct LocationIndex {
2019-07-25T09:20:40.5561621Z 21  | |       pub struct LocationIndex {
2019-07-25T09:20:40.5561912Z 22  | |           DEBUG_FORMAT = "LocationIndex({})"
2019-07-25T09:20:40.5562869Z 24  | |   }
2019-07-25T09:20:40.5563153Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.5563201Z 
2019-07-25T09:20:40.5563201Z 
2019-07-25T09:20:40.5563460Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5564127Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.5564399Z     |
2019-07-25T09:20:40.5564758Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.5565093Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.5565480Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.5565815Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.5566075Z ...     |
2019-07-25T09:20:40.5566409Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.5566794Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.5567201Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.5567975Z ...     |
2019-07-25T09:20:40.5567975Z ...     |
2019-07-25T09:20:40.5568293Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.5568910Z ...     |
2019-07-25T09:20:40.5568910Z ...     |
2019-07-25T09:20:40.5569229Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.5569738Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5570008Z ...     |
2019-07-25T09:20:40.5570341Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.5570623Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.5571281Z     |   |                    |
2019-07-25T09:20:40.5571281Z     |   |                    |
2019-07-25T09:20:40.5571595Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.5572085Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.5572365Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.5572798Z    ::: src/librustc_mir/borrow_check/location.rs:20:1
2019-07-25T09:20:40.5572994Z     |
2019-07-25T09:20:40.5573260Z 20  | /   newtype_index! {
2019-07-25T09:20:40.5573538Z 21  | |       pub struct LocationIndex {
2019-07-25T09:20:40.5573538Z 21  | |       pub struct LocationIndex {
2019-07-25T09:20:40.5573818Z 22  | |           DEBUG_FORMAT = "LocationIndex({})"
2019-07-25T09:20:40.5574779Z 24  | |   }
2019-07-25T09:20:40.5575082Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.5575132Z 
2019-07-25T09:20:40.5575132Z 
2019-07-25T09:20:40.5898483Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.5903453Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.5908987Z     |
2019-07-25T09:20:40.5914301Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.5919428Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.5952923Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.5953307Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.5953578Z ...     |
2019-07-25T09:20:40.5956358Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.5956848Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.5957239Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.5958422Z ...     |
2019-07-25T09:20:40.5958422Z ...     |
2019-07-25T09:20:40.5961891Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6011395Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.6011756Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.6012088Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.6012438Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6012701Z ...     |
2019-07-25T09:20:40.6013035Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6013326Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6014800Z     |   |                    |
2019-07-25T09:20:40.6014800Z     |   |                    |
2019-07-25T09:20:40.6015217Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6015577Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6015908Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6016448Z    ::: src/librustc_mir/borrow_check/nll/region_infer/values.rs:119:1
2019-07-25T09:20:40.6016666Z     |
2019-07-25T09:20:40.6016978Z 119 | /   newtype_index! {
2019-07-25T09:20:40.6016978Z 119 | /   newtype_index! {
2019-07-25T09:20:40.6017330Z 120 | |       /// A single integer representing a `Location` in the MIR control-flow
2019-07-25T09:20:40.6017846Z 121 | |       /// graph. Constructed efficiently from `RegionValueElements`.
2019-07-25T09:20:40.6018343Z 122 | |       pub struct PointIndex { DEBUG_FORMAT = "PointIndex({})" }
2019-07-25T09:20:40.6018955Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6018991Z 
2019-07-25T09:20:40.6018991Z 
2019-07-25T09:20:40.6019244Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6019521Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.6019718Z     |
2019-07-25T09:20:40.6020148Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6020471Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6020799Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6021103Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6021346Z ...     |
2019-07-25T09:20:40.6021645Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6022015Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6022365Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6022940Z ...     |
2019-07-25T09:20:40.6022940Z ...     |
2019-07-25T09:20:40.6023257Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6024400Z ...     |
2019-07-25T09:20:40.6024400Z ...     |
2019-07-25T09:20:40.6025558Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.6025968Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6026243Z ...     |
2019-07-25T09:20:40.6026601Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6026953Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6027895Z     |   |                    |
2019-07-25T09:20:40.6027895Z     |   |                    |
2019-07-25T09:20:40.6028221Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6028529Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6028834Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6029389Z    ::: src/librustc_mir/borrow_check/nll/region_infer/values.rs:119:1
2019-07-25T09:20:40.6029608Z     |
2019-07-25T09:20:40.6029874Z 119 | /   newtype_index! {
2019-07-25T09:20:40.6029874Z 119 | /   newtype_index! {
2019-07-25T09:20:40.6030340Z 120 | |       /// A single integer representing a `Location` in the MIR control-flow
2019-07-25T09:20:40.6030659Z 121 | |       /// graph. Constructed efficiently from `RegionValueElements`.
2019-07-25T09:20:40.6030961Z 122 | |       pub struct PointIndex { DEBUG_FORMAT = "PointIndex({})" }
2019-07-25T09:20:40.6031503Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6031538Z 
2019-07-25T09:20:40.6031538Z 
2019-07-25T09:20:40.6031786Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6032056Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.6032245Z     |
2019-07-25T09:20:40.6032551Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6032848Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6033169Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6033488Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6033707Z ...     |
2019-07-25T09:20:40.6034398Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6034812Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6035189Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6035975Z ...     |
2019-07-25T09:20:40.6035975Z ...     |
2019-07-25T09:20:40.6036330Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6036761Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.6037094Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.6037696Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.6038039Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6038290Z ...     |
2019-07-25T09:20:40.6038603Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6038899Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6039431Z     |   |                    |
2019-07-25T09:20:40.6039431Z     |   |                    |
2019-07-25T09:20:40.6039750Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6040048Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6040340Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6041129Z    ::: src/librustc_mir/borrow_check/nll/region_infer/values.rs:125:1
2019-07-25T09:20:40.6041336Z     |
2019-07-25T09:20:40.6041598Z 125 | /   newtype_index! {
2019-07-25T09:20:40.6041598Z 125 | /   newtype_index! {
2019-07-25T09:20:40.6041899Z 126 | |       /// A single integer representing a `ty::Placeholder`.
2019-07-25T09:20:40.6122811Z 127 | |       pub struct PlaceholderIndex { DEBUG_FORMAT = "PlaceholderIndex({})" }
2019-07-25T09:20:40.6123394Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6123434Z 
2019-07-25T09:20:40.6123434Z 
2019-07-25T09:20:40.6123687Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6124521Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.6124780Z     |
2019-07-25T09:20:40.6125309Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6125703Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6126069Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6126406Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6126681Z ...     |
2019-07-25T09:20:40.6127129Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6127526Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6128082Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6128644Z ...     |
2019-07-25T09:20:40.6128644Z ...     |
2019-07-25T09:20:40.6129119Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6129718Z ...     |
2019-07-25T09:20:40.6129718Z ...     |
2019-07-25T09:20:40.6130035Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.6130376Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6130607Z ...     |
2019-07-25T09:20:40.6131337Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6131614Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6132333Z     |   |                    |
2019-07-25T09:20:40.6132333Z     |   |                    |
2019-07-25T09:20:40.6132646Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6133113Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6133394Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6134753Z    ::: src/librustc_mir/borrow_check/nll/region_infer/values.rs:125:1
2019-07-25T09:20:40.6134971Z     |
2019-07-25T09:20:40.6135286Z 125 | /   newtype_index! {
2019-07-25T09:20:40.6135286Z 125 | /   newtype_index! {
2019-07-25T09:20:40.6135620Z 126 | |       /// A single integer representing a `ty::Placeholder`.
2019-07-25T09:20:40.6135973Z 127 | |       pub struct PlaceholderIndex { DEBUG_FORMAT = "PlaceholderIndex({})" }
2019-07-25T09:20:40.6136697Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6136739Z 
2019-07-25T09:20:40.6136739Z 
2019-07-25T09:20:40.6161655Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6169129Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.6172432Z     |
2019-07-25T09:20:40.6180197Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6180681Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6180963Z     |  _|_-
2019-07-25T09:20:40.6181313Z 3   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6181665Z 4   | | | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6182225Z ...     |
2019-07-25T09:20:40.6182225Z ...     |
2019-07-25T09:20:40.6182547Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6182933Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.6183256Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.6183563Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.6184431Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6184720Z ...     |
2019-07-25T09:20:40.6185095Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6185631Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6186314Z     |   |                    |
2019-07-25T09:20:40.6186314Z     |   |                    |
2019-07-25T09:20:40.6186651Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6186992Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6187471Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6188173Z    ::: src/librustc_mir/borrow_check/nll/type_check/liveness/local_use_map.rs:47:1
2019-07-25T09:20:40.6188372Z     |
2019-07-25T09:20:40.6188638Z 47  | /   newtype_index! {
2019-07-25T09:20:40.6188939Z 48  | |       pub struct AppearanceIndex { .. }
2019-07-25T09:20:40.6188939Z 48  | |       pub struct AppearanceIndex { .. }
2019-07-25T09:20:40.6189197Z 49  | |   }
2019-07-25T09:20:40.6189465Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6189529Z 
2019-07-25T09:20:40.6207481Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6208971Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.6243040Z     |
2019-07-25T09:20:40.6244784Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6245232Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6245531Z     |  _|_-
2019-07-25T09:20:40.6245972Z 3   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6246375Z 4   | | | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6246987Z ...     |
2019-07-25T09:20:40.6246987Z ...     |
2019-07-25T09:20:40.6247369Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6248202Z ...     |
2019-07-25T09:20:40.6248202Z ...     |
2019-07-25T09:20:40.6248704Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.6249122Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6249380Z ...     |
2019-07-25T09:20:40.6249710Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6250019Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6250696Z     |   |                    |
2019-07-25T09:20:40.6250696Z     |   |                    |
2019-07-25T09:20:40.6251040Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6251364Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6251667Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6252171Z    ::: src/librustc_mir/borrow_check/nll/type_check/liveness/local_use_map.rs:47:1
2019-07-25T09:20:40.6252385Z     |
2019-07-25T09:20:40.6252677Z 47  | /   newtype_index! {
2019-07-25T09:20:40.6252980Z 48  | |       pub struct AppearanceIndex { .. }
2019-07-25T09:20:40.6252980Z 48  | |       pub struct AppearanceIndex { .. }
2019-07-25T09:20:40.6253264Z 49  | |   }
2019-07-25T09:20:40.6253550Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6253589Z 
2019-07-25T09:20:40.6432510Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6436703Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.6440798Z     |
2019-07-25T09:20:40.6444869Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6448437Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6458665Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6459013Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6459246Z ...     |
2019-07-25T09:20:40.6459569Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6460085Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6460491Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6461223Z ...     |
2019-07-25T09:20:40.6461223Z ...     |
2019-07-25T09:20:40.6461574Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6462071Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.6462411Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.6462731Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.6463084Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6463339Z ...     |
2019-07-25T09:20:40.6463672Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6464429Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6465063Z     |   |                    |
2019-07-25T09:20:40.6465063Z     |   |                    |
2019-07-25T09:20:40.6465421Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6465764Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6466091Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6466628Z    ::: src/librustc_mir/borrow_check/nll/constraints/mod.rs:103:1
2019-07-25T09:20:40.6466846Z     |
2019-07-25T09:20:40.6467166Z 103 | /   newtype_index! {
2019-07-25T09:20:40.6467166Z 103 | /   newtype_index! {
2019-07-25T09:20:40.6467706Z 104 | |       pub struct OutlivesConstraintIndex {
2019-07-25T09:20:40.6468034Z 105 | |           DEBUG_FORMAT = "OutlivesConstraintIndex({})"
2019-07-25T09:20:40.6468578Z 107 | |   }
2019-07-25T09:20:40.6468876Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6469019Z 
2019-07-25T09:20:40.6469019Z 
2019-07-25T09:20:40.6526687Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6527029Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.6527484Z     |
2019-07-25T09:20:40.6527801Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6528270Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6530454Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6530807Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6531050Z ...     |
2019-07-25T09:20:40.6531344Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6531679Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6532021Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6532587Z ...     |
2019-07-25T09:20:40.6532587Z ...     |
2019-07-25T09:20:40.6535564Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6536306Z ...     |
2019-07-25T09:20:40.6536306Z ...     |
2019-07-25T09:20:40.6536678Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.6537074Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6537502Z ...     |
2019-07-25T09:20:40.6537812Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6538101Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6538639Z     |   |                    |
2019-07-25T09:20:40.6538639Z     |   |                    |
2019-07-25T09:20:40.6539081Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6539427Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6539726Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6540166Z    ::: src/librustc_mir/borrow_check/nll/constraints/mod.rs:103:1
2019-07-25T09:20:40.6540370Z     |
2019-07-25T09:20:40.6540627Z 103 | /   newtype_index! {
2019-07-25T09:20:40.6540627Z 103 | /   newtype_index! {
2019-07-25T09:20:40.6541002Z 104 | |       pub struct OutlivesConstraintIndex {
2019-07-25T09:20:40.6541492Z 105 | |           DEBUG_FORMAT = "OutlivesConstraintIndex({})"
2019-07-25T09:20:40.6542013Z 107 | |   }
2019-07-25T09:20:40.6542302Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6542339Z 
2019-07-25T09:20:40.6542339Z 
2019-07-25T09:20:40.6542595Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6542882Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.6543077Z     |
2019-07-25T09:20:40.6543406Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6543715Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6544447Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6544809Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6545070Z ...     |
2019-07-25T09:20:40.6545404Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6545814Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6546232Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6546866Z ...     |
2019-07-25T09:20:40.6546866Z ...     |
2019-07-25T09:20:40.6547233Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6548108Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.6548443Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.6548745Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.6549105Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6549419Z ...     |
2019-07-25T09:20:40.6549768Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6550057Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6550618Z     |   |                    |
2019-07-25T09:20:40.6550618Z     |   |                    |
2019-07-25T09:20:40.6551079Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6551396Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6551692Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6552147Z    ::: src/librustc_mir/borrow_check/nll/constraints/mod.rs:109:1
2019-07-25T09:20:40.6552337Z     |
2019-07-25T09:20:40.6552591Z 109 | /   newtype_index! {
2019-07-25T09:20:40.6552882Z 110 | |       pub struct ConstraintSccIndex {
2019-07-25T09:20:40.6552882Z 110 | |       pub struct ConstraintSccIndex {
2019-07-25T09:20:40.6553170Z 111 | |           DEBUG_FORMAT = "ConstraintSccIndex({})"
2019-07-25T09:20:40.6553704Z 113 | |   }
2019-07-25T09:20:40.6554335Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6554380Z 
2019-07-25T09:20:40.6554380Z 
2019-07-25T09:20:40.6554697Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6554992Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.6555210Z     |
2019-07-25T09:20:40.6555582Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6555918Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6556372Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6556766Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6557015Z ...     |
2019-07-25T09:20:40.6557368Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6557905Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6558329Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6558889Z ...     |
2019-07-25T09:20:40.6558889Z ...     |
2019-07-25T09:20:40.6559215Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6559860Z ...     |
2019-07-25T09:20:40.6559860Z ...     |
2019-07-25T09:20:40.6560194Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.6560535Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6560770Z ...     |
2019-07-25T09:20:40.6561080Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6561355Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6562478Z     |   |                    |
2019-07-25T09:20:40.6562478Z     |   |                    |
2019-07-25T09:20:40.6562793Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6563111Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6563391Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6563847Z    ::: src/librustc_mir/borrow_check/nll/constraints/mod.rs:109:1
2019-07-25T09:20:40.6564434Z     |
2019-07-25T09:20:40.6564756Z 109 | /   newtype_index! {
2019-07-25T09:20:40.6565194Z 110 | |       pub struct ConstraintSccIndex {
2019-07-25T09:20:40.6565194Z 110 | |       pub struct ConstraintSccIndex {
2019-07-25T09:20:40.6565565Z 111 | |           DEBUG_FORMAT = "ConstraintSccIndex({})"
2019-07-25T09:20:40.6566162Z 113 | |   }
2019-07-25T09:20:40.6566464Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6566607Z 
2019-07-25T09:20:40.6566607Z 
2019-07-25T09:20:40.6566913Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6567213Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:124:9
2019-07-25T09:20:40.6567452Z     |
2019-07-25T09:20:40.6567941Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6568221Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6568557Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6568848Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6569088Z ...     |
2019-07-25T09:20:40.6569389Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6569728Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6570074Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6570637Z ...     |
2019-07-25T09:20:40.6570637Z ...     |
2019-07-25T09:20:40.6570952Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6571313Z     |   |                  --------------------------------------------------- in this macro invocation (#3)
2019-07-25T09:20:40.6571621Z 123 |   | @ decodable $ type : ident ) => (
2019-07-25T09:20:40.6571916Z 124 |   | impl :: rustc_serialize :: Decodable for $ type {
2019-07-25T09:20:40.6572256Z     |   |         ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6572485Z ...     |
2019-07-25T09:20:40.6572864Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6573186Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6573712Z     |   |                    |
2019-07-25T09:20:40.6573712Z     |   |                    |
2019-07-25T09:20:40.6574413Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6574917Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6575264Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6575768Z    ::: src/librustc_mir/borrow_check/nll/member_constraints.rs:54:1
2019-07-25T09:20:40.6576003Z     |
2019-07-25T09:20:40.6576293Z 54  | /   newtype_index! {
2019-07-25T09:20:40.6576293Z 54  | /   newtype_index! {
2019-07-25T09:20:40.6576616Z 55  | |       crate struct NllMemberConstraintIndex {
2019-07-25T09:20:40.6576978Z 56  | |           DEBUG_FORMAT = "MemberConstraintIndex({})"
2019-07-25T09:20:40.6577707Z 58  | |   }
2019-07-25T09:20:40.6577989Z     | |___- in this macro invocation (#1)
2019-07-25T09:20:40.6578074Z 
2019-07-25T09:20:40.6578074Z 
2019-07-25T09:20:40.6578349Z error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6578607Z    --> <::rustc_data_structures::indexed_vec::newtype_index macros>:125:20
2019-07-25T09:20:40.6578796Z     |
2019-07-25T09:20:40.6579131Z 1   |   / ( $ ( # [ $ attrs : meta ] ) * $ v : vis struct $ name : ident { .. } ) => (
2019-07-25T09:20:40.6579417Z 2   |   | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6579737Z 3   |   | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6580045Z 4   |   | vis [ $ v ] @ debug_format [ "{}" ] ) ; ) ; (
2019-07-25T09:20:40.6580258Z ...     |
2019-07-25T09:20:40.6580562Z 7   | / | $ crate :: newtype_index ! (
2019-07-25T09:20:40.6580908Z 8   | | | @ attrs [ $ ( # [ $ attrs ] ) * ] @ type [ $ name ] @ max [ 0xFFFF_FF00 ] @
2019-07-25T09:20:40.6581311Z 9   | | | vis [ $ v ] @ debug_format [ "{}" ] $ ( $ tokens ) + ) ; ) ; (
2019-07-25T09:20:40.6581900Z ...     |
2019-07-25T09:20:40.6581900Z ...     |
2019-07-25T09:20:40.6582225Z 122 |   | $ tokens ) * ) ; $ crate :: newtype_index ! ( @ decodable $ type ) ; ) ; (
2019-07-25T09:20:40.6582903Z ...     |
2019-07-25T09:20:40.6582903Z ...     |
2019-07-25T09:20:40.6583235Z 125 |   | fn decode < D : :: rustc_serialize :: Decoder > ( d : & mut D ) -> Result <
2019-07-25T09:20:40.6583572Z     |   |                    ^^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
2019-07-25T09:20:40.6583806Z ...     |
2019-07-25T09:20:40.6584584Z 167 |   | $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ (
2019-07-25T09:20:40.6584917Z 168 |   | $ tokens ) * ) ; ) ;
2019-07-25T09:20:40.6585556Z     |   |                    |
2019-07-25T09:20:40.6585556Z     |   |                    |
2019-07-25T09:20:40.6585897Z     |   |                    in this expansion of `newtype_index!` (#1)
2019-07-25T09:20:40.6586258Z     |   |____________________in this expansion of `$crate::newtype_index!` (#2)
2019-07-25T09:20:40.6586579Z     |                        in this expansion of `$crate::newtype_index!` (#3)
2019-07-25T09:20:40.6587581Z    ::: src/librustc_mir/borrow_check/nll/member_constraints.rs:54:1
2019-07-25T09:20:40.6587798Z     |
2019-07-25T09:20:40.6588115Z 54  | /   newtype_index! {
2019-07-25T09:20:40.6588115Z 54  | /   newtype_index! {
2019-07-25T09:20:40.6588443Z 55  | |       crate struct NllMemberConstraintIndex {
2019-07-25T09:20:40.6588776Z 56  | |           DEBUG_FORMAT = "MemberConstraintIndex({})"
2019-07-25T09:20:40.6589372Z 58  | |   }
2019-07-25T09:20:40.6589686Z     | |___- in this macro invocation (#1)
