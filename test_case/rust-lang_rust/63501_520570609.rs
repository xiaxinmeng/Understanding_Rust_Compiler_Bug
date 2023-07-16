plain
2019-08-12T19:21:04.3750659Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T19:21:04.3940317Z ##[command]git config gc.auto 0
2019-08-12T19:21:04.4025359Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T19:21:04.4084916Z ##[command]git config --get-all http.proxy
2019-08-12T19:21:04.4240003Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63501/merge:refs/remotes/pull/63501/merge
---
2019-08-12T19:21:41.9655643Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T19:21:41.9655672Z 
2019-08-12T19:21:41.9655861Z   git checkout -b <new-branch-name>
2019-08-12T19:21:41.9655905Z 
2019-08-12T19:21:41.9655950Z HEAD is now at a89b721c2 Merge 18e54539ca08ee9cad46200206320ce290086999 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-12T19:21:41.9811877Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T19:21:41.9814832Z ==============================================================================
2019-08-12T19:21:41.9814913Z Task         : Bash
2019-08-12T19:21:41.9814964Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T19:53:19.5774080Z Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-12T19:53:19.8390164Z    Compiling unicode-width v0.1.5
2019-08-12T19:53:19.8393235Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-08-12T19:53:19.9566397Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-08-12T19:53:20.0880052Z error[E0496]: lifetime name `'s` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.0888446Z    --> src/libproc_macro/bridge/client.rs:83:51
2019-08-12T19:53:20.0899641Z 5   | / macro_rules! define_handles {
2019-08-12T19:53:20.0905084Z 6   | |     (
2019-08-12T19:53:20.0905084Z 6   | |     (
2019-08-12T19:53:20.0910638Z 7   | |         'owned: $($oty:ident,)*
2019-08-12T19:53:20.0916394Z 8   | |         'interned: $($ity:ident,)*
2019-08-12T19:53:20.1012622Z ...   |
2019-08-12T19:53:20.1013127Z 80  | |             impl<S: server::Types> Decode<'_, 's, HandleStore<server::MarkedTypes<S>>>
2019-08-12T19:53:20.1013570Z     | |                                               -- first declared here
2019-08-12T19:53:20.1013860Z ...   |
2019-08-12T19:53:20.1014304Z 83  | |                 fn decode(r: &mut Reader<'_>, s: &'s HandleStore<server::MarkedTypes<S>>) -> Self {
2019-08-12T19:53:20.1014968Z     | |                                                   ^^ lifetime 's already in scope
2019-08-12T19:53:20.1015669Z 155 | |     }
2019-08-12T19:53:20.1015987Z 156 | | }
2019-08-12T19:53:20.1015987Z 156 | | }
2019-08-12T19:53:20.1016360Z     | |_- in this expansion of `define_handles!`
2019-08-12T19:53:20.1016692Z 157 | / define_handles! {
2019-08-12T19:53:20.1017027Z 158 | |     'owned:
2019-08-12T19:53:20.1017379Z 159 | |     TokenStream,
2019-08-12T19:53:20.1017716Z 160 | |     TokenStreamBuilder,
2019-08-12T19:53:20.1018331Z 171 | |     Span,
2019-08-12T19:53:20.1018645Z 172 | | }
2019-08-12T19:53:20.1018989Z     | |_- in this macro invocation
2019-08-12T19:53:20.1019035Z 
2019-08-12T19:53:20.1019035Z 
2019-08-12T19:53:20.1019354Z error[E0496]: lifetime name `'s` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1019662Z    --> src/libproc_macro/bridge/client.rs:99:25
2019-08-12T19:53:20.1020262Z 5   | / macro_rules! define_handles {
2019-08-12T19:53:20.1020678Z 6   | |     (
2019-08-12T19:53:20.1020678Z 6   | |     (
2019-08-12T19:53:20.1021089Z 7   | |         'owned: $($oty:ident,)*
2019-08-12T19:53:20.1021766Z 8   | |         'interned: $($ity:ident,)*
2019-08-12T19:53:20.1022127Z ...   |
2019-08-12T19:53:20.1024181Z 94  | |             impl<S: server::Types> DecodeMut<'_, 's, HandleStore<server::MarkedTypes<S>>>
2019-08-12T19:53:20.1024785Z     | |                                                  -- first declared here
2019-08-12T19:53:20.1025099Z ...   |
2019-08-12T19:53:20.1025488Z 99  | |                     s: &'s mut HandleStore<server::MarkedTypes<S>>
2019-08-12T19:53:20.1025882Z     | |                         ^^ lifetime 's already in scope
2019-08-12T19:53:20.1026492Z 155 | |     }
2019-08-12T19:53:20.1074900Z 156 | | }
2019-08-12T19:53:20.1074900Z 156 | | }
2019-08-12T19:53:20.1075462Z     | |_- in this expansion of `define_handles!`
2019-08-12T19:53:20.1075799Z 157 | / define_handles! {
2019-08-12T19:53:20.1076164Z 158 | |     'owned:
2019-08-12T19:53:20.1076506Z 159 | |     TokenStream,
2019-08-12T19:53:20.1076841Z 160 | |     TokenStreamBuilder,
2019-08-12T19:53:20.1077463Z 171 | |     Span,
2019-08-12T19:53:20.1077770Z 172 | | }
2019-08-12T19:53:20.1078120Z     | |_- in this macro invocation
2019-08-12T19:53:20.1078165Z 
2019-08-12T19:53:20.1078165Z 
2019-08-12T19:53:20.1078485Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1078839Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1079078Z     |
2019-08-12T19:53:20.1079417Z 26  | / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1079769Z 27  | |     (le $ty:ty) => {
2019-08-12T19:53:20.1080127Z 28  | |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1080499Z 29  | |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1080797Z ...   |
2019-08-12T19:53:20.1081345Z 82  | |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1082264Z     | |                                           -- first declared here
2019-08-12T19:53:20.1082559Z ...   |
2019-08-12T19:53:20.1082946Z 85  | |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1083376Z     | |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1083968Z 104 | |     }
2019-08-12T19:53:20.1084309Z 105 | | }
2019-08-12T19:53:20.1084309Z 105 | | }
2019-08-12T19:53:20.1084654Z     | |_- in this expansion of `rpc_encode_decode!`
2019-08-12T19:53:20.1084884Z ...
2019-08-12T19:53:20.1085236Z 187 | / rpc_encode_decode!(
2019-08-12T19:53:20.1085735Z 188 | |     enum Bound<T> {
2019-08-12T19:53:20.1086085Z 189 | |         Included(x),
2019-08-12T19:53:20.1086422Z 190 | |         Excluded(x),
2019-08-12T19:53:20.1086759Z 191 | |         Unbounded,
2019-08-12T19:53:20.1087412Z 193 | | );
2019-08-12T19:53:20.1087739Z     | |__- in this macro invocation
2019-08-12T19:53:20.1087812Z 
2019-08-12T19:53:20.1087812Z 
2019-08-12T19:53:20.1088134Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1088424Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1088681Z     |
2019-08-12T19:53:20.1089023Z 26  | / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1089364Z 27  | |     (le $ty:ty) => {
2019-08-12T19:53:20.1089730Z 28  | |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1090105Z 29  | |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1090385Z ...   |
2019-08-12T19:53:20.1090798Z 82  | |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1091203Z     | |                                           -- first declared here
2019-08-12T19:53:20.1091926Z ...   |
2019-08-12T19:53:20.1092428Z 85  | |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1092837Z     | |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1093464Z 104 | |     }
2019-08-12T19:53:20.1093772Z 105 | | }
2019-08-12T19:53:20.1093772Z 105 | | }
2019-08-12T19:53:20.1094143Z     | |_- in this expansion of `rpc_encode_decode!`
2019-08-12T19:53:20.1094379Z ...
2019-08-12T19:53:20.1094702Z 195 | / rpc_encode_decode!(
2019-08-12T19:53:20.1095057Z 196 | |     enum Option<T> {
2019-08-12T19:53:20.1095858Z 198 | |         Some(x),
2019-08-12T19:53:20.1096203Z 199 | |     }
2019-08-12T19:53:20.1096511Z 200 | | );
2019-08-12T19:53:20.1096857Z     | |__- in this macro invocation
2019-08-12T19:53:20.1096857Z     | |__- in this macro invocation
2019-08-12T19:53:20.1096909Z 
2019-08-12T19:53:20.1097230Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1097518Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1097780Z     |
2019-08-12T19:53:20.1098116Z 26  | / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1098458Z 27  | |     (le $ty:ty) => {
2019-08-12T19:53:20.1098828Z 28  | |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1099196Z 29  | |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1099492Z ...   |
2019-08-12T19:53:20.1099889Z 82  | |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1100294Z     | |                                           -- first declared here
2019-08-12T19:53:20.1100665Z ...   |
2019-08-12T19:53:20.1101034Z 85  | |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1101438Z     | |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1102596Z 104 | |     }
2019-08-12T19:53:20.1102930Z 105 | | }
2019-08-12T19:53:20.1102930Z 105 | | }
2019-08-12T19:53:20.1103283Z     | |_- in this expansion of `rpc_encode_decode!`
2019-08-12T19:53:20.1103523Z ...
2019-08-12T19:53:20.1103864Z 202 | / rpc_encode_decode!(
2019-08-12T19:53:20.1104203Z 203 | |     enum Result<T, E> {
2019-08-12T19:53:20.1104527Z 204 | |         Ok(x),
2019-08-12T19:53:20.1104885Z 205 | |         Err(e),
2019-08-12T19:53:20.1105509Z 207 | | );
2019-08-12T19:53:20.1105859Z     | |__- in this macro invocation
2019-08-12T19:53:20.1105990Z 
2019-08-12T19:53:20.1105990Z 
2019-08-12T19:53:20.1106331Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1106852Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1153441Z     |
2019-08-12T19:53:20.1153855Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1154265Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1154632Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1156183Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1156527Z ...     |
2019-08-12T19:53:20.1156933Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1157411Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1157699Z ...     |
2019-08-12T19:53:20.1158081Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1158528Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1159156Z 104 |   |     }
2019-08-12T19:53:20.1159487Z 105 |   | }
2019-08-12T19:53:20.1159487Z 105 |   | }
2019-08-12T19:53:20.1160049Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1160380Z     | 
2019-08-12T19:53:20.1160674Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1160913Z     |
2019-08-12T19:53:20.1161271Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1162130Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1162862Z 54  |  |          $m! {
2019-08-12T19:53:20.1163193Z     |  |______________-
2019-08-12T19:53:20.1163565Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1163957Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1164401Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1169345Z 159 | ||              },
2019-08-12T19:53:20.1169679Z 160 | ||          }
2019-08-12T19:53:20.1170040Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1170395Z 161 |  |      };
2019-08-12T19:53:20.1170395Z 161 |  |      };
2019-08-12T19:53:20.1170715Z 162 |  |  }
2019-08-12T19:53:20.1171066Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1171995Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1172419Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1172419Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1172845Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1173188Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1173483Z ...   |
2019-08-12T19:53:20.1173855Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1174609Z ...   |
2019-08-12T19:53:20.1174924Z 243 | |           }
2019-08-12T19:53:20.1175388Z 244 | |       }
2019-08-12T19:53:20.1175388Z 244 | |       }
2019-08-12T19:53:20.1175791Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1176121Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1176585Z 
2019-08-12T19:53:20.1176585Z 
2019-08-12T19:53:20.1176907Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1177215Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1177458Z     |
2019-08-12T19:53:20.1177814Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1178179Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1178542Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1179051Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1179353Z ...     |
2019-08-12T19:53:20.1179749Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1180223Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1180512Z ...     |
2019-08-12T19:53:20.1180892Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1181343Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1182309Z 104 |   |     }
2019-08-12T19:53:20.1182673Z 105 |   | }
2019-08-12T19:53:20.1182673Z 105 |   | }
2019-08-12T19:53:20.1183038Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1183280Z     | 
2019-08-12T19:53:20.1183590Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1183825Z     |
2019-08-12T19:53:20.1184171Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1184560Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1184893Z 54  |  |          $m! {
2019-08-12T19:53:20.1185392Z     |  |______________-
2019-08-12T19:53:20.1185803Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1186190Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1186984Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1187740Z 159 | ||              },
2019-08-12T19:53:20.1188107Z 160 | ||          }
2019-08-12T19:53:20.1188451Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1188849Z 161 |  |      };
2019-08-12T19:53:20.1188849Z 161 |  |      };
2019-08-12T19:53:20.1189175Z 162 |  |  }
2019-08-12T19:53:20.1189709Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1190301Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1190638Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1190638Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1191076Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1191418Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1192060Z ...   |
2019-08-12T19:53:20.1192512Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1193254Z ...   |
2019-08-12T19:53:20.1193578Z 243 | |           }
2019-08-12T19:53:20.1193894Z 244 | |       }
2019-08-12T19:53:20.1193894Z 244 | |       }
2019-08-12T19:53:20.1194269Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1194597Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1195062Z 
2019-08-12T19:53:20.1195062Z 
2019-08-12T19:53:20.1195381Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1195794Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1196104Z     |
2019-08-12T19:53:20.1196451Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1196792Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1197175Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1197565Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1197856Z ...     |
2019-08-12T19:53:20.1198266Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1198712Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1199015Z ...     |
2019-08-12T19:53:20.1199393Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1199953Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1200564Z 104 |   |     }
2019-08-12T19:53:20.1200907Z 105 |   | }
2019-08-12T19:53:20.1200907Z 105 |   | }
2019-08-12T19:53:20.1201276Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1201519Z     | 
2019-08-12T19:53:20.1202211Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1202475Z     |
2019-08-12T19:53:20.1202813Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1203205Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1203547Z 54  |  |          $m! {
2019-08-12T19:53:20.1203840Z     |  |______________-
2019-08-12T19:53:20.1204225Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1204680Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1205123Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1205890Z 159 | ||              },
2019-08-12T19:53:20.1206285Z 160 | ||          }
2019-08-12T19:53:20.1206628Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1206970Z 161 |  |      };
2019-08-12T19:53:20.1206970Z 161 |  |      };
2019-08-12T19:53:20.1207309Z 162 |  |  }
2019-08-12T19:53:20.1207661Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1208247Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1208591Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1208591Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1209019Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1209359Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1209749Z ...   |
2019-08-12T19:53:20.1210151Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1210890Z ...   |
2019-08-12T19:53:20.1211215Z 243 | |           }
2019-08-12T19:53:20.1211529Z 244 | |       }
2019-08-12T19:53:20.1211529Z 244 | |       }
2019-08-12T19:53:20.1212361Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1212720Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1213178Z 
2019-08-12T19:53:20.1213178Z 
2019-08-12T19:53:20.1213510Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1213803Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1214066Z     |
2019-08-12T19:53:20.1214412Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1214754Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1215145Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1215523Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1215814Z ...     |
2019-08-12T19:53:20.1216334Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1216821Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1217125Z ...     |
2019-08-12T19:53:20.1217516Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1217932Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1218563Z 104 |   |     }
2019-08-12T19:53:20.1218902Z 105 |   | }
2019-08-12T19:53:20.1218902Z 105 |   | }
2019-08-12T19:53:20.1219269Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1219629Z     | 
2019-08-12T19:53:20.1219932Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1220175Z     |
2019-08-12T19:53:20.1220515Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1220908Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1221250Z 54  |  |          $m! {
2019-08-12T19:53:20.1221541Z     |  |______________-
2019-08-12T19:53:20.1222313Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1222721Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1223154Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1223816Z 159 | ||              },
2019-08-12T19:53:20.1224157Z 160 | ||          }
2019-08-12T19:53:20.1224505Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1224842Z 161 |  |      };
2019-08-12T19:53:20.1224842Z 161 |  |      };
2019-08-12T19:53:20.1225187Z 162 |  |  }
2019-08-12T19:53:20.1225537Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1226246Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1268768Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1268768Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1307803Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1308397Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1308684Z ...   |
2019-08-12T19:53:20.1309082Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1309821Z ...   |
2019-08-12T19:53:20.1310164Z 243 | |           }
2019-08-12T19:53:20.1310480Z 244 | |       }
2019-08-12T19:53:20.1310480Z 244 | |       }
2019-08-12T19:53:20.1311083Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1311423Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1312278Z 
2019-08-12T19:53:20.1312278Z 
2019-08-12T19:53:20.1312607Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1312900Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1313160Z     |
2019-08-12T19:53:20.1313514Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1313871Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1314246Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1314634Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1314915Z ...     |
2019-08-12T19:53:20.1315339Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1315785Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1316088Z ...     |
2019-08-12T19:53:20.1316473Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1317034Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1317707Z 104 |   |     }
2019-08-12T19:53:20.1318027Z 105 |   | }
2019-08-12T19:53:20.1318027Z 105 |   | }
2019-08-12T19:53:20.1318417Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1318672Z     | 
2019-08-12T19:53:20.1318951Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1319212Z     |
2019-08-12T19:53:20.1319548Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1319939Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1320283Z 54  |  |          $m! {
2019-08-12T19:53:20.1320574Z     |  |______________-
2019-08-12T19:53:20.1320947Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1321464Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1322258Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1322967Z 159 | ||              },
2019-08-12T19:53:20.1323309Z 160 | ||          }
2019-08-12T19:53:20.1323660Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1323996Z 161 |  |      };
2019-08-12T19:53:20.1323996Z 161 |  |      };
2019-08-12T19:53:20.1324334Z 162 |  |  }
2019-08-12T19:53:20.1324702Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1325296Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1325632Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1325632Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1326032Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1326408Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1326675Z ...   |
2019-08-12T19:53:20.1327182Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1327946Z ...   |
2019-08-12T19:53:20.1328288Z 243 | |           }
2019-08-12T19:53:20.1328615Z 244 | |       }
2019-08-12T19:53:20.1328615Z 244 | |       }
2019-08-12T19:53:20.1328967Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1329320Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1329760Z 
2019-08-12T19:53:20.1329760Z 
2019-08-12T19:53:20.1330102Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1330394Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1330629Z     |
2019-08-12T19:53:20.1331119Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1331463Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1332158Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1332591Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1332868Z ...     |
2019-08-12T19:53:20.1333283Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1333747Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1334023Z ...     |
2019-08-12T19:53:20.1334428Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1334853Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1335524Z 104 |   |     }
2019-08-12T19:53:20.1335843Z 105 |   | }
2019-08-12T19:53:20.1335843Z 105 |   | }
2019-08-12T19:53:20.1336233Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1336484Z     | 
2019-08-12T19:53:20.1336764Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1337021Z     |
2019-08-12T19:53:20.1337470Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1337879Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1338243Z 54  |  |          $m! {
2019-08-12T19:53:20.1338547Z     |  |______________-
2019-08-12T19:53:20.1338906Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1339319Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1339738Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1340410Z 159 | ||              },
2019-08-12T19:53:20.1340730Z 160 | ||          }
2019-08-12T19:53:20.1341216Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1341911Z 161 |  |      };
2019-08-12T19:53:20.1341911Z 161 |  |      };
2019-08-12T19:53:20.1342245Z 162 |  |  }
2019-08-12T19:53:20.1342637Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1343223Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1343567Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1343567Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1343978Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1344343Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1344608Z ...   |
2019-08-12T19:53:20.1344993Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1345735Z ...   |
2019-08-12T19:53:20.1346083Z 243 | |           }
2019-08-12T19:53:20.1346404Z 244 | |       }
2019-08-12T19:53:20.1346404Z 244 | |       }
2019-08-12T19:53:20.1365559Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1367422Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1367884Z 
2019-08-12T19:53:20.1367884Z 
2019-08-12T19:53:20.1368171Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1368422Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1368635Z     |
2019-08-12T19:53:20.1368944Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1369238Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1369544Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1369888Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1370118Z ...     |
2019-08-12T19:53:20.1370451Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1370986Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1371219Z ...     |
2019-08-12T19:53:20.1371561Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1372489Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1373066Z 104 |   |     }
2019-08-12T19:53:20.1373372Z 105 |   | }
2019-08-12T19:53:20.1373372Z 105 |   | }
2019-08-12T19:53:20.1373718Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1373944Z     | 
2019-08-12T19:53:20.1374203Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1374434Z     |
2019-08-12T19:53:20.1374741Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1375181Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
2019-08-12T19:53:20.1375477Z 54  |  |          $m! {
2019-08-12T19:53:20.1375734Z     |  |______________-
2019-08-12T19:53:20.1376032Z 55  | ||              TokenStream {
2019-08-12T19:53:20.1376483Z 56  | ||                  fn drop($self: $S::TokenStream);
2019-08-12T19:53:20.1376870Z 57  | ||                  fn clone($self: &$S::TokenStream) -> $S::TokenStream;
2019-08-12T19:53:20.1377438Z 159 | ||              },
2019-08-12T19:53:20.1377718Z 160 | ||          }
2019-08-12T19:53:20.1378022Z     | ||__________- in this macro invocation (#2)
2019-08-12T19:53:20.1378299Z 161 |  |      };
2019-08-12T19:53:20.1378299Z 161 |  |      };
2019-08-12T19:53:20.1378573Z 162 |  |  }
2019-08-12T19:53:20.1378894Z     |  |__- in this expansion of `with_api!` (#1)
2019-08-12T19:53:20.1379366Z 227 | /       macro_rules! declare_tags {
2019-08-12T19:53:20.1379779Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1379779Z 228 | |           ($($name:ident {
2019-08-12T19:53:20.1380117Z 229 | |               $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
2019-08-12T19:53:20.1380421Z 230 | |           }),* $(,)?) => {
2019-08-12T19:53:20.1380657Z ...   |
2019-08-12T19:53:20.1380971Z 235 | |                   rpc_encode_decode!(enum $name { $($method),* });
2019-08-12T19:53:20.1381990Z ...   |
2019-08-12T19:53:20.1382350Z 243 | |           }
2019-08-12T19:53:20.1382661Z 244 | |       }
2019-08-12T19:53:20.1382661Z 244 | |       }
2019-08-12T19:53:20.1382986Z     | |_______- in this expansion of `declare_tags!` (#2)
2019-08-12T19:53:20.1383304Z 245 |         with_api!(self, self, declare_tags);
2019-08-12T19:53:20.1383701Z 
2019-08-12T19:53:20.1383701Z 
2019-08-12T19:53:20.1383984Z error[E0496]: lifetime name `'a` shadows a lifetime name that is already in scope
2019-08-12T19:53:20.1384278Z    --> src/libproc_macro/bridge/rpc.rs:85:38
2019-08-12T19:53:20.1384490Z     |
2019-08-12T19:53:20.1384798Z 26  |   / macro_rules! rpc_encode_decode {
2019-08-12T19:53:20.1385448Z 27  |   |     (le $ty:ty) => {
2019-08-12T19:53:20.1385784Z 28  |   |         impl<S> Encode<S> for $ty {
2019-08-12T19:53:20.1386122Z 29  |   |             fn encode(self, w: &mut Writer, _: &mut S) {
2019-08-12T19:53:20.1386356Z ...     |
2019-08-12T19:53:20.1388122Z 82  |   |         impl<S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
2019-08-12T19:53:20.1388565Z     |   |                                                                      -- first declared here
2019-08-12T19:53:20.1388820Z ...     |
2019-08-12T19:53:20.1389174Z 85  |   |             fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
2019-08-12T19:53:20.1389547Z     |   |                                      ^^ lifetime 'a already in scope
2019-08-12T19:53:20.1390257Z 104 |   |     }
2019-08-12T19:53:20.1390656Z 105 |   | }
2019-08-12T19:53:20.1390656Z 105 |   | }
2019-08-12T19:53:20.1390958Z     |   |_- in this expansion of `rpc_encode_decode!` (#3)
2019-08-12T19:53:20.1391196Z     | 
2019-08-12T19:53:20.1391434Z    ::: src/libproc_macro/bridge/mod.rs:52:1
2019-08-12T19:53:20.1391631Z     |
2019-08-12T19:53:20.1392473Z 52  |  /  macro_rules! with_api {
2019-08-12T19:53:20.1392805Z 53  |  |      ($S:ident, $self:ident, $m:ident) => {
---
2019-08-12T19:53:25.3073249Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
2019-08-12T19:53:25.3076728Z expected success, got: exit code: 101
2019-08-12T19:53:25.3087096Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-12T19:53:25.3087370Z Build completed unsuccessfully in 0:25:05
2019-08-12T19:53:27.2110489Z ##[error]Bash exited with code '1'.
2019-08-12T19:53:27.2178561Z ##[section]Starting: Checkout
2019-08-12T19:53:27.2180658Z ==============================================================================
2019-08-12T19:53:27.2180746Z Task         : Get sources
2019-08-12T19:53:27.2180801Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
