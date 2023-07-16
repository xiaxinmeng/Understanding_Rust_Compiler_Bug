
[dilbert-feed]$ cargo +9898131620287f170f82e89c851e0d7fc8436424 build

[...]
Finished dev [unoptimized + debuginfo] target(s) in 15.71s


[dilbert-feed]$ cargo +beta build

[...]

error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:37:32: could not fully normalize `<tokio::prelude::future::Either<tokio::prelude::future::Map<tokio::prelude::future::AndThen<tokio::prelude::future::AndThen<tokio::prelude::future::FutureResult<std::string::String, failure::Error>, std::result::Result<hyper::Uri, failure::Error>, [closure@src/main.rs:162:51: 162:96]>, tokio::prelude::future::Map<tokio::prelude::future::FromErr<tokio::prelude::future::AndThen<hyper::client::ResponseFuture, tokio::prelude::future::Map<tokio::prelude::stream::Concat2<hyper::Body>, [closure@src/main.rs:46:40: 46:62]>, fn(hyper::Response<hyper::Body>) -> impl hyper::rt::Future {concat_body}>, failure::Error>, [closure@src/main.rs:91:14: 95:10]>, [closure@src/main.rs:163:51: 163:92]>, [closure@src/main.rs:164:46: 167:42]>, tokio::prelude::future::FutureResult<ComicInfo, failure::Error>> as tokio::prelude::IntoFuture>::Future`
