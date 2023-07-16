
error[E0275]: overflow evaluating the requirement `_: std::io::Read`
  --> src/main.rs:22:35
   |
22 |     let mut st: Something<File> = Something::new(file);
   |                                   ^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because of the requirements on the impl of `std::io::Read` for `&tar::archive::ArchiveInner<_>`
   = note: required because of the requirements on the impl of `std::io::Read` for `&tokio_reactor::poll_evented::PollEvented<tar::archive::ArchiveInner<_>>`
   = note: required because of the requirements on the impl of `std::io::Read` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tar::archive::ArchiveInner<_>>>`
   = note: required because of the requirements on the impl of `std::io::Read` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tar::archive::ArchiveInner<_>>>>`
   = note: required because of the requirements on the impl of `std::io::Read` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tar::archive::ArchiveInner<_>>>>>`

