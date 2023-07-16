plain
   Compiling proc-macro-srv v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-srv)
error[E0725]: the feature `proc_macro_internals` is not in the list of allowed features
  --> crates/proc-macro-srv/src/lib.rs:14:47
   |
14 | #![cfg_attr(feature = "in-rust-tree", feature(proc_macro_internals))]

error[E0725]: the feature `proc_macro_diagnostic` is not in the list of allowed features
  --> crates/proc-macro-srv/src/lib.rs:15:47
   |
   |
15 | #![cfg_attr(feature = "in-rust-tree", feature(proc_macro_diagnostic))]

error[E0725]: the feature `proc_macro_span` is not in the list of allowed features
  --> crates/proc-macro-srv/src/lib.rs:16:47
   |
   |
16 | #![cfg_attr(feature = "in-rust-tree", feature(proc_macro_span))]

error[E0437]: type `Ident` is not a member of trait `server::Types`
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:282:5
    |
    |
282 |     type Ident = IdentId;
    |     ^^^^^^^^^^^^^^^^^^^^^ not a member of trait `server::Types`

error[E0437]: type `Literal` is not a member of trait `server::Types`
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:283:5
283 |     type Literal = Literal;
    |     ^^^^^^^^^^^^^^^^^^^^^^^ not a member of trait `server::Types`

error[E0404]: expected trait, found struct `server::Ident`
---
    |
note: these traits exist but are inaccessible
   --> crates/proc-macro-srv/src/abis/abi_1_58/proc_macro/bridge/server.rs:50:11
    |
42  | /   macro_rules! declare_server_traits {
43  | |       ($($name:ident {
44  | |           $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
45  | |       }),* $(,)?) => {
...   |
50  | |           $(pub trait $name: Types {
    | |             ^^^^^^^^^^^^^^^^^^^^^^ `crate::abis::abi_1_58::proc_macro::bridge::server::Ident`: not accessible
56  | |       }
57  | |   }
57  | |   }
    | |___- in this expansion of `declare_server_traits!` (#2)
58  |     with_api!(Self, self_, declare_server_traits);
    |
   ::: crates/proc-macro-srv/src/abis/abi_1_63/proc_macro/bridge/server.rs:33:1
    |
33  |  /  macro_rules! declare_server_traits {
33  |  /  macro_rules! declare_server_traits {
34  |  |      ($($name:ident {
35  |  |          $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
36  |  |      }),* $(,)?) => {
37  |  |          $(pub trait $name: Types {
    |  |            ^^^^^^^^^^^^^^^^^^^^^^ `crate::abis::abi_1_63::proc_macro::bridge::server::Ident`: not accessible
43  |  |      }
44  |  |  }
44  |  |  }
    |  |__- in this expansion of `declare_server_traits!` (#2)
45  |     with_api!(Self, self_, declare_server_traits);
    |
   ::: crates/proc-macro-srv/src/abis/abi_1_64/proc_macro/bridge/server.rs:31:1
    |
31  |   / macro_rules! declare_server_traits {
31  |   / macro_rules! declare_server_traits {
32  |   |     ($($name:ident {
33  |   |         $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
34  |   |     }),* $(,)?) => {
35  |   |         $(pub trait $name: Types {
    |   |           ^^^^^^^^^^^^^^^^^^^^^^ `crate::abis::abi_1_64::proc_macro::bridge::server::Ident`: not accessible
42  |   |     }
43  |   | }
43  |   | }
    |   |_- in this expansion of `declare_server_traits!` (#2)
44  |     with_api!(Self, self_, declare_server_traits);
    |
   ::: crates/proc-macro-srv/src/abis/abi_1_64/proc_macro/bridge/mod.rs:52:1
    |
    |
52  | /   macro_rules! with_api {
53  | |       ($S:ident, $self:ident, $m:ident) => {
54  |             $m! {
55  |                 FreeFunctions {
55  |                 FreeFunctions {
56  |                     fn drop($self: $S::FreeFunctions);
57  |                     fn track_env_var(var: &str, value: Option<&str>);
148 |                 },
149 | |           }
    | |___________- in this macro invocation (#2)
150 | |       };
150 | |       };
151 | |   }
    | |___- in this expansion of `with_api!` (#1)
   ::: crates/proc-macro-srv/src/abis/abi_1_63/proc_macro/bridge/mod.rs:52:1
    |
    |
52  | /   macro_rules! with_api {
53  | |       ($S:ident, $self:ident, $m:ident) => {
54  |             $m! {
55  |                 FreeFunctions {
55  |                 FreeFunctions {
56  |                     fn drop($self: $S::FreeFunctions);
57  |                     fn track_env_var(var: &str, value: Option<&str>);
169 |                 },
170 | |           }
    | |___________- in this macro invocation (#2)
171 | |       };
171 | |       };
172 | |   }
    | |___- in this expansion of `with_api!` (#1)
   ::: crates/proc-macro-srv/src/abis/abi_1_58/proc_macro/bridge/mod.rs:52:1
    |
    |
52  | /   macro_rules! with_api {
53  | |       ($S:ident, $self:ident, $m:ident) => {
54  |             $m! {
55  |                 FreeFunctions {
55  |                 FreeFunctions {
56  |                     fn drop($self: $S::FreeFunctions);
57  |                     fn track_env_var(var: &str, value: Option<&str>);
173 |                 },
174 | |           }
    | |___________- in this macro invocation (#2)
175 | |       };
175 | |       };
176 | |   }
    | |___- in this expansion of `with_api!` (#1)
error[E0404]: expected trait, found struct `server::Literal`
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:465:6
    |
465 | impl server::Literal for Rustc {
465 | impl server::Literal for Rustc {
    |      ^^^^^^^^^^^^^^^ not a trait
    |
note: these traits exist but are inaccessible
   --> crates/proc-macro-srv/src/abis/abi_1_58/proc_macro/bridge/server.rs:50:11
    |
42  | /   macro_rules! declare_server_traits {
43  | |       ($($name:ident {
44  | |           $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
45  | |       }),* $(,)?) => {
...   |
50  | |           $(pub trait $name: Types {
    | |             ^^^^^^^^^^^^^^^^^^^^^^ `crate::abis::abi_1_58::proc_macro::bridge::server::Literal`: not accessible
56  | |       }
57  | |   }
57  | |   }
    | |___- in this expansion of `declare_server_traits!` (#2)
58  |     with_api!(Self, self_, declare_server_traits);
    |
   ::: crates/proc-macro-srv/src/abis/abi_1_63/proc_macro/bridge/server.rs:33:1
    |
33  |  /  macro_rules! declare_server_traits {
33  |  /  macro_rules! declare_server_traits {
34  |  |      ($($name:ident {
35  |  |          $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
36  |  |      }),* $(,)?) => {
37  |  |          $(pub trait $name: Types {
    |  |            ^^^^^^^^^^^^^^^^^^^^^^ `crate::abis::abi_1_63::proc_macro::bridge::server::Literal`: not accessible
43  |  |      }
44  |  |  }
44  |  |  }
    |  |__- in this expansion of `declare_server_traits!` (#2)
45  |     with_api!(Self, self_, declare_server_traits);
    |
   ::: crates/proc-macro-srv/src/abis/abi_1_64/proc_macro/bridge/server.rs:31:1
    |
31  |   / macro_rules! declare_server_traits {
31  |   / macro_rules! declare_server_traits {
32  |   |     ($($name:ident {
33  |   |         $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
34  |   |     }),* $(,)?) => {
35  |   |         $(pub trait $name: Types {
    |   |           ^^^^^^^^^^^^^^^^^^^^^^ `crate::abis::abi_1_64::proc_macro::bridge::server::Literal`: not accessible
42  |   |     }
43  |   | }
43  |   | }
    |   |_- in this expansion of `declare_server_traits!` (#2)
44  |     with_api!(Self, self_, declare_server_traits);
    |
   ::: crates/proc-macro-srv/src/abis/abi_1_64/proc_macro/bridge/mod.rs:52:1
    |
    |
52  | /   macro_rules! with_api {
53  | |       ($S:ident, $self:ident, $m:ident) => {
54  |             $m! {
55  |                 FreeFunctions {
55  |                 FreeFunctions {
56  |                     fn drop($self: $S::FreeFunctions);
57  |                     fn track_env_var(var: &str, value: Option<&str>);
148 |                 },
149 | |           }
    | |___________- in this macro invocation (#2)
150 | |       };
150 | |       };
151 | |   }
    | |___- in this expansion of `with_api!` (#1)
   ::: crates/proc-macro-srv/src/abis/abi_1_63/proc_macro/bridge/mod.rs:52:1
    |
    |
52  | /   macro_rules! with_api {
53  | |       ($S:ident, $self:ident, $m:ident) => {
54  |             $m! {
55  |                 FreeFunctions {
55  |                 FreeFunctions {
56  |                     fn drop($self: $S::FreeFunctions);
57  |                     fn track_env_var(var: &str, value: Option<&str>);
169 |                 },
170 | |           }
    | |___________- in this macro invocation (#2)
171 | |       };
171 | |       };
172 | |   }
    | |___- in this expansion of `with_api!` (#1)
   ::: crates/proc-macro-srv/src/abis/abi_1_58/proc_macro/bridge/mod.rs:52:1
    |
    |
52  | /   macro_rules! with_api {
53  | |       ($S:ident, $self:ident, $m:ident) => {
54  |             $m! {
55  |                 FreeFunctions {
55  |                 FreeFunctions {
56  |                     fn drop($self: $S::FreeFunctions);
57  |                     fn track_env_var(var: &str, value: Option<&str>);
173 |                 },
174 | |           }
    | |___________- in this macro invocation (#2)
175 | |       };
175 | |       };
176 | |   }
    | |___- in this expansion of `with_api!` (#1)
error[E0603]: struct `Ident` is private
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:450:14
    |
450 | impl server::Ident for Rustc {
---
    |
3   | use super::*;
    |     ^^^^^^^^

error[E0603]: struct `Literal` is private
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:465:14
465 | impl server::Literal for Rustc {
    |              ^^^^^^^ private struct
    |
    |
note: the struct `Literal` is defined here
   --> /checkout/library/proc_macro/src/bridge/server.rs:3:5
3   | use super::*;
    |     ^^^^^^^^

error[E0658]: use of unstable library feature 'proc_macro_internals'
error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:15:26
   |
15 |     exported_macros: Vec<proc_macro::bridge::client::ProcMacro>,
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:18:11
   |
18 | impl From<proc_macro::bridge::PanicMessage> for PanicMessage {
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:19:16
   |
19 |     fn from(p: proc_macro::bridge::PanicMessage) -> Self {
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:26:47
   |
26 |         let macros: libloading::Symbol<'_, &&[proc_macro::bridge::client::ProcMacro]> =
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:45:17
   |
45 |                 proc_macro::bridge::client::ProcMacro::CustomDerive {
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:49:26
   |
49 |                         &proc_macro::bridge::server::SameThread,
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:56:17
   |
56 |                 proc_macro::bridge::client::ProcMacro::Bang { name, client }
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:60:26
   |
60 |                         &proc_macro::bridge::server::SameThread,
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:67:17
   |
67 |                 proc_macro::bridge::client::ProcMacro::Attr { name, client }
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:71:26
   |
71 |                         &proc_macro::bridge::server::SameThread,
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:83:13
   |
83 |         Err(proc_macro::bridge::PanicMessage::String("Nothing to expand".to_string()).into())
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:90:17
   |
90 |                 proc_macro::bridge::client::ProcMacro::CustomDerive { trait_name, .. } => {
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:93:17
   |
93 |                 proc_macro::bridge::client::ProcMacro::Bang { name, .. } => {
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/mod.rs:96:17
   |
96 |                 proc_macro::bridge::client::ProcMacro::Attr { name, .. } => {
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:13:14
   |
13 |     bridge::{self, server},
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
  --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:13:20
   |
13 |     bridge::{self, server},
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_diagnostic'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:108:14
    |
108 | type Level = super::proc_macro::Level;
    |
    = note: see issue #54140 <https://github.com/rust-lang/rust/issues/54140> for more information
    = help: add `#![feature(proc_macro_diagnostic)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_span'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:109:19
    |
109 | type LineColumn = super::proc_macro::LineColumn;
    |
    = note: see issue #54725 <https://github.com/rust-lang/rust/issues/54725> for more information
    = note: see issue #54725 <https://github.com/rust-lang/rust/issues/54725> for more information
    = help: add `#![feature(proc_macro_span)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:280:5
    |
280 |     type FreeFunctions = FreeFunctions;
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:286:5
    |
286 |     type Span = Span;
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:287:5
    |
287 |     type MultiSpan = Vec<Span>;
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:291:5
    |
291 | /     fn track_env_var(&mut self, _var: &str, _value: Option<&str>) {
292 | |         // FIXME: track env var accesses
293 | |         // https://github.com/rust-lang/rust/pull/71858
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:295:5
    |
295 |     fn track_path(&mut self, _path: &str) {}
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:299:5
    |
299 | /     fn is_empty(&mut self, stream: &Self::TokenStream) -> bool {
300 | |         stream.is_empty()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:302:5
    |
302 | /     fn from_str(&mut self, src: &str) -> Self::TokenStream {
303 | |         use std::str::FromStr;
304 | |
305 | |         Self::TokenStream::from_str(src).expect("cannot parse string")
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:307:5
    |
307 | /     fn to_string(&mut self, stream: &Self::TokenStream) -> String {
308 | |         stream.to_string()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:310:5
    |
310 | /     fn from_token_tree(
311 | |         &mut self,
312 | |         tree: bridge::TokenTree<Self::TokenStream, Self::Span, Self::Ident, Self::Literal>,
313 | |     ) -> Self::TokenStream {
348 | |         }
349 | |     }
    | |_____^
    |
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:351:5
    |
351 | /     fn expand_expr(&mut self, self_: &Self::TokenStream) -> Result<Self::TokenStream, ()> {
352 | |         Ok(self_.clone())
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:355:5
    |
355 | /     fn concat_trees(
356 | |         &mut self,
357 | |         base: Option<Self::TokenStream>,
358 | |         trees: Vec<bridge::TokenTree<Self::TokenStream, Self::Span, Self::Ident, Self::Literal>>,
367 | |         builder.build()
368 | |     }
    | |_____^
    |
---
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:370:5
    |
370 | /     fn concat_streams(
371 | |         &mut self,
372 | |         base: Option<Self::TokenStream>,
373 | |         streams: Vec<Self::TokenStream>,
382 | |         builder.build()
383 | |     }
    | |_____^
    |
---
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:385:5
    |
385 | /     fn into_trees(
386 | |         &mut self,
387 | |         stream: Self::TokenStream,
388 | |     ) -> Vec<bridge::TokenTree<Self::TokenStream, Self::Span, Self::Ident, Self::Literal>> {
413 | |             .collect()
414 | |     }
    | |_____^
    |
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:578:5
    |
578 | /     fn eq(&mut self, _file1: &Self::SourceFile, _file2: &Self::SourceFile) -> bool {
580 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:584:5
    |
584 | /     fn is_real(&mut self, _file: &Self::SourceFile) -> bool {
586 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:590:5
    |
590 | /     fn new(&mut self, level: Level, msg: &str, spans: Self::MultiSpan) -> Self::Diagnostic {
591 | |         let mut diag = Diagnostic::new(level, msg);
592 | |         diag.spans = spans;
594 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:596:5
    |
596 | /     fn sub(
597 | |         &mut self,
598 | |         _diag: &mut Self::Diagnostic,
599 | |         _level: Level,
604 | |         //
605 | |     }
    | |_____^
    |
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:607:5
    |
607 | /     fn emit(&mut self, _diag: Self::Diagnostic) {
608 | |         // FIXME handle diagnostic
609 | |         // diag.emit()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:614:5
    |
614 | /     fn debug(&mut self, span: Self::Span) -> String {
615 | |         format!("{:?}", span.0)
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:617:5
    |
617 | /     fn source_file(&mut self, _span: Self::Span) -> Self::SourceFile {
619 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:620:5
    |
620 | /     fn save_span(&mut self, _span: Self::Span) -> usize {
621 | |         // FIXME stub
623 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:624:5
    |
624 | /     fn recover_proc_macro_span(&mut self, _id: usize) -> Self::Span {
625 | |         // FIXME stub
626 | |         tt::TokenId::unspecified()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:632:5
    |
632 | /     fn source_text(&mut self, _span: Self::Span) -> Option<String> {
634 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:636:5
    |
636 | /     fn parent(&mut self, _span: Self::Span) -> Option<Self::Span> {
637 | |         // FIXME handle span
639 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:640:5
    |
640 | /     fn source(&mut self, span: Self::Span) -> Self::Span {
641 | |         // FIXME handle span
643 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:644:5
    |
644 | /     fn start(&mut self, _span: Self::Span) -> LineColumn {
645 | |         // FIXME handle span
646 | |         LineColumn { line: 0, column: 0 }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:648:5
    |
648 | /     fn end(&mut self, _span: Self::Span) -> LineColumn {
649 | |         // FIXME handle span
650 | |         LineColumn { line: 0, column: 0 }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:652:5
    |
652 | /     fn join(&mut self, first: Self::Span, _second: Self::Span) -> Option<Self::Span> {
653 | |         // Just return the first span again, because some macros will unwrap the result.
654 | |         Some(first)
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:656:5
    |
656 | /     fn resolved_at(&mut self, _span: Self::Span, _at: Self::Span) -> Self::Span {
657 | |         // FIXME handle span
658 | |         tt::TokenId::unspecified()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:661:5
    |
661 | /     fn after(&mut self, _self_: Self::Span) -> Self::Span {
662 | |         tt::TokenId::unspecified()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:665:5
    |
665 | /     fn before(&mut self, _self_: Self::Span) -> Self::Span {
666 | |         tt::TokenId::unspecified()
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:676:5
    |
676 | /     fn push(&mut self, other: &mut Self::MultiSpan, span: Self::Span) {
677 | |         //TODP
678 | |         other.push(span)
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:683:5
    |
683 | /     fn globals(&mut self) -> bridge::ExpnGlobals<Self::Span> {
684 | |         bridge::ExpnGlobals {
685 | |             def_site: tt::TokenId::unspecified(),
686 | |             call_site: tt::TokenId::unspecified(),
687 | |             mixed_site: tt::TokenId::unspecified(),
689 | |     }
    | |_____^
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
---

error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:312:15
    |
312 |         tree: bridge::TokenTree<Self::TokenStream, Self::Span, Self::Ident, Self::Literal>,
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:315:13
    |
315 |             bridge::TokenTree::Group(group) => {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:324:13
    |
324 |             bridge::TokenTree::Ident(IdentId(index)) => {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:332:13
    |
332 |             bridge::TokenTree::Literal(literal) => {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:338:13
    |
338 |             bridge::TokenTree::Punct(p) => {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:358:20
    |
358 |         trees: Vec<bridge::TokenTree<Self::TokenStream, Self::Span, Self::Ident, Self::Literal>>,
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:388:14
    |
388 |     ) -> Vec<bridge::TokenTree<Self::TokenStream, Self::Span, Self::Ident, Self::Literal>> {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:393:21
    |
393 |                     bridge::TokenTree::Ident(IdentId(self.ident_interner.intern(&IdentData(ident))))
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:395:64
    |
395 |                 tt::TokenTree::Leaf(tt::Leaf::Literal(lit)) => bridge::TokenTree::Literal(lit),
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:397:21
    |
397 |                     bridge::TokenTree::Punct(bridge::Punct {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:397:46
    |
397 |                     bridge::TokenTree::Punct(bridge::Punct {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:403:52
    |
403 |                 tt::TokenTree::Subtree(subtree) => bridge::TokenTree::Group(bridge::Group {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:403:77
    |
403 |                 tt::TokenTree::Subtree(subtree) => bridge::TokenTree::Group(bridge::Group {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:406:27
    |
406 |                     span: bridge::DelimSpan {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:683:30
    |
683 |     fn globals(&mut self) -> bridge::ExpnGlobals<Self::Span> {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'proc_macro_internals'
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:684:9
    |
684 |         bridge::ExpnGlobals {
    |
    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
    = help: add `#![feature(proc_macro_internals)]` to the crate attributes to enable


Some errors have detailed explanations: E0404, E0437, E0603, E0658, E0725.
For more information about an error, try `rustc --explain E0404`.
error: could not compile `proc-macro-srv` due to 89 previous errors
warning: build failed, waiting for other jobs to finish...
thread 'main' panicked at 'in-tree tool', test.rs:382:14
Build completed unsuccessfully in 0:27:55
