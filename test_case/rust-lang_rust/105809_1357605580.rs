
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:969:13: failed to get layout for `&mut Pin<&mut 
server::shutdown::Graceful<rocket::http::private::Incoming<rocket::ext::CancellableListener<rocket::Shutdown, L>>, 
service::make::MakeServiceFn<[closure@rocket::server::<impl Rocket<Orbit>>::http_server<L>::{closure#0}::
{closure#2}]>, rocket::Shutdown, hyper::common::exec::Exec>>`: unable to determine layout for `&mut Pin<&mut 
server::shutdown::Graceful<rocket::http::private::Incoming<rocket::ext::CancellableListener<rocket::Shutdown, L>>, 
service::make::MakeServiceFn<[closure@rocket::server::<impl Rocket<Orbit>>::http_server<L>::{closure#0}::
{closure#2}]>, rocket::Shutdown, hyper::common::exec::Exec>>` because `&mut Pin<&mut 
server::shutdown::Graceful<rocket::http::private::Incoming<rocket::ext::CancellableListener<rocket::Shutdown, L>>, 
service::make::MakeServiceFn<[closure@rocket::server::<impl Rocket<Orbit>>::http_server<L>::{closure#0}::
{closure#2}]>, rocket::Shutdown, hyper::common::exec::Exec>>` cannot be normalized

...
note: rustc 1.68.0-nightly (9c07efe84 2022-12-16) running on x86_64-unknown-linux-gnu)
