plain
travis_time:end:0abdab18:start=1543599199125516414,finish=1543599201583208281,duration=2457691867
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:19:47]    Compiling getopts v0.2.17
[00:19:47]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:19:47]    Compiling term v0.0.0 (/checkout/src/libterm)
[00:19:49] error[E0599]: no method named `encode` found for type `<std::collections::Bound<T> as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49] ...
[00:19:49] ...
[00:19:49] 201 | / rpc_encode_decode!(
[00:19:49] 202 | |     enum Bound<T> {
[00:19:49] 203 | |         Included(x),
[00:19:49] 204 | |         Excluded(x),
[00:19:49] 205 | |         Unbounded,
[00:19:49] 207 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<std::option::Option<T> as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49] ...
[00:19:49] ...
[00:19:49] 209 | / rpc_encode_decode!(
[00:19:49] 210 | |     enum Option<T> {
[00:19:49] 212 | |         Some(x),
[00:19:49] 213 | |     }
[00:19:49] 214 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<std::result::Result<T, E> as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49] ...
[00:19:49] ...
[00:19:49] 216 | / rpc_encode_decode!(
[00:19:49] 217 | |     enum Result<T, E> {
[00:19:49] 218 | |         Ok(x),
[00:19:49] 219 | |         Err(e),
[00:19:49] 221 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::TokenStream as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::TokenStreamBuilder as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::TokenStreamIter as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Group as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Punct as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Ident as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Literal as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::SourceFile as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::MultiSpan as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Diagnostic as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Span as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::api_tags::Method as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                 #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                             -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                         <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `bridge::<impl bridge::rpc::Encode<S> for Delimiter>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:351:1
[00:19:49]     |
[00:19:49] 351 | / rpc_encode_decode!(
[00:19:49] 352 | |     enum Delimiter {
[00:19:49] 353 | |         Parenthesis,
[00:19:49] ...   |
[00:19:49] 357 | |     }
[00:19:49] 358 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `bridge::<impl bridge::rpc::Encode<S> for diagnostic::Level>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:359:1
[00:19:49]     |
[00:19:49] 359 | / rpc_encode_decode!(
[00:19:49] 360 | |     enum Level {
[00:19:49] 362 | |         Warning,
[00:19:49] ...   |
[00:19:49] 365 | |     }
[00:19:49] 366 | | );
[00:19:49] 366 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `bridge::<impl bridge::rpc::Encode<S> for Spacing>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:368:1
[00:19:49]     |
[00:19:49] 368 | / rpc_encode_decode!(
[00:19:49] 369 | |     enum Spacing {
[00:19:49] 370 | |         Alone,
[00:19:49] 371 | |         Joint,
[00:19:49] 373 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] 
[00:19:49] error[E0599]: no method named `encode` found for type `<bridge::TokenTree<G, P, I, L> as bridge::rpc::Encode<S>>::encode::Tag` in the current scope
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:92:41
[00:19:49]     |
[00:19:49] 86  |                   #[repr(u8)] enum Tag { $($variant),* }
[00:19:49]     |                               -------- method `encode` not found for this
[00:19:49] ...
[00:19:49] 92  |                           <Tag>::$variant.encode(w, s);
[00:19:49]     | 
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:406:1
[00:19:49]     |
[00:19:49] 406 | / rpc_encode_decode!(
[00:19:49] 407 | |     enum TokenTree<G, P, I, L> {
[00:19:49] 408 | |         Group(tt),
[00:19:49] 409 | |         Punct(tt),
[00:19:49] 412 | |     }
[00:19:49] 413 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     |
[00:19:49]     = help: items from traits can only be used if the trait is implemented and in scope
[00:19:49]     = note: the following trait defines an item `encode`, perhaps you need to implement it:
[00:19:49]             candidate #1: `bridge::rpc::Encode`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                       $(<Tag>::$variant => {
[00:19:49]     |                         ^^^^^^^^^^^^^^^ expected u8, found enum `<std::collections::Bound<T> as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] ...
[00:19:49] 201 | / rpc_encode_decode!(
[00:19:49] 202 | |     enum Bound<T> {
[00:19:49] 203 | |         Included(x),
[00:19:49] 204 | |         Excluded(x),
[00:19:49] 205 | |         Unbounded,
[00:19:49] 207 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<std::collections::Bound<T> as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                       $(<Tag>::$variant => {
[00:19:49]     |                         ^^^^^^^^^^^^^^^ expected u8, found enum `<std::option::Option<T> as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] ...
[00:19:49] 209 | / rpc_encode_decode!(
[00:19:49] 210 | |     enum Option<T> {
[00:19:49] 212 | |         Some(x),
[00:19:49] 213 | |     }
[00:19:49] 214 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<std::option::Option<T> as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                       $(<Tag>::$variant => {
[00:19:49]     |                         ^^^^^^^^^^^^^^^ expected u8, found enum `<std::result::Result<T, E> as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] ...
[00:19:49] 216 | / rpc_encode_decode!(
[00:19:49] 217 | |     enum Result<T, E> {
[00:19:49] 218 | |         Ok(x),
[00:19:49] 219 | |         Err(e),
[00:19:49] 221 | | );
[00:19:49]     | |__- in this macro invocation
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<std::result::Result<T, E> as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                     $(<Tag>::$variant => {
[00:19:49]     |                       ^^^^^^^^^^^^^^^ expected u8, found enum `<bridge::api_tags::TokenStream as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<bridge::api_tags::TokenStream as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                     $(<Tag>::$variant => {
[00:19:49]     |                       ^^^^^^^^^^^^^^^ expected u8, found enum `<bridge::api_tags::TokenStreamBuilder as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<bridge::api_tags::TokenStreamBuilder as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                     $(<Tag>::$variant => {
[00:19:49]     |                       ^^^^^^^^^^^^^^^ expected u8, found enum `<bridge::api_tags::TokenStreamIter as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<bridge::api_tags::TokenStreamIter as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                     $(<Tag>::$variant => {
[00:19:49]     |                       ^^^^^^^^^^^^^^^ expected u8, found enum `<bridge::api_tags::Group as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<bridge::api_tags::Group as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49] error[E0308]: mismatched types
[00:19:49] error[E0308]: mismatched types
[00:19:49]    --> src/libproc_macro/bridge/rpc.rs:110:23
[00:19:49]     |
[00:19:49] 110 |                     $(<Tag>::$variant => {
[00:19:49]     |                       ^^^^^^^^^^^^^^^ expected u8, found enum `<bridge::api_tags::Punct as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
[00:19:49]     | 
[00:19:49]    ::: src/libproc_macro/bridge/mod.rs:254:5
[00:19:49]     |
[00:19:49] 254 |     with_api!(self, self, declare_tags);
[00:19:49]     |
[00:19:49]     = note: expected type `u8`
[00:19:49]     = note: expected type `u8`
[00:19:49]                found type `<bridge::api_tags::Punct as bridge::rpc::DecodeMut<'a, '_, S>>::decode::Tag`
---
153284 ./src/tools/clang
149728 ./obj/build/bootstrap/debug/incremental
146748 ./.git
134136 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134132 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f75rq7ao59-sxl4qz-2xn3crcwe77k0
131136 ./.git/modules/src
115344 ./src/llvm/test/CodeGen
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
