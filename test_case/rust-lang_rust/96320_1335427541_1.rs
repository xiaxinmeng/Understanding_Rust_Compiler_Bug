
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `Message<SomeRequest>: core::marker::Send`
#1 [codegen_select_candidate] computing candidate for `<core::pin::Pin<alloc::boxed::Box<core::future::from_generator::GenFuture<[static generator@src/main.rs:...]>>> as core::ops::unsize::CoerceUnsized<core::pin::Pin<alloc::boxed::Box<dyn core::future::future::Future<Output = core::result::Result<tonic::response::Response<tokio_stream::wrappers::mpsc_bounded::ReceiverStream<core::result::Result<SomeReply, tonic::status::Status>>>, tonic::status::Status>> + core::marker::Send>>>>`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
#3 [exported_symbols] exported_symbols
