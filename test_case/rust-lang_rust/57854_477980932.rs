sh
❯ cargo build
   Compiling proj v0.2.0 (../proj)
error[E0275]: overflow evaluating the requirement `&tokio_reactor::poll_evented::PollEvented<_>: std::io::Write`
  |
  = help: consider adding a `#![recursion_limit="10"]` attribute to your crate
  = note: required because of the requirements on the impl of `std::io::Write` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<_>>`
  = note: required because of the requirements on the impl of `std::io::Write` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<_>>>`
  = note: required because of the requirements on the impl of `std::io::Write` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<_>>>>`
  = note: required because of the requirements on the impl of `tokio_io::async_write::AsyncWrite` for `&tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<tokio_reactor::poll_evented::PollEvented<_>>>>>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
error: Could not compile `proj`.

To learn more, run the command again with --verbose.
