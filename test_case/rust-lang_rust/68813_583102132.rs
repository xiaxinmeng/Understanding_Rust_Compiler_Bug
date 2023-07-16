
% cargo +nightly build
warning: unnecessary parentheses around block return value
   --> /private/tmp/rust-lightning/lightning/src/chain/keysinterface.rs:449:3
    |
449 |         (Sha256::from_engine(sha).into_inner())
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
    |
    = note: `#[warn(unused_parens)]` on by default

warning: use of deprecated item 'std::error::Error::description': use the Display impl or to_string()
   --> /private/tmp/rust-lightning/lightning/src/ln/msgs.rs:741:32
    |
741 |             DecodeError::Io(ref e) => e.description(),
    |                                         ^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default

warning: use of deprecated item 'std::error::Error::description': use the Display impl or to_string()
   --> /private/tmp/rust-lightning/lightning/src/ln/msgs.rs:747:20
    |
747 |         f.write_str(self.description())
    |                          ^^^^^^^^^^^

warning: field is never read: `logger`
   --> /private/tmp/rust-lightning/lightning/src/chain/chaininterface.rs:303:2
    |
303 |     logger: Arc<Logger>,
    |     ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

warning: field is never read: `logger`
   --> /private/tmp/rust-lightning/lightning/src/chain/keysinterface.rs:295:2
    |
295 |     logger: Arc<Logger>,
    |     ^^^^^^^^^^^^^^^^^^^

warning: variant is never constructed: `Watchtower`
   --> /private/tmp/rust-lightning/lightning/src/ln/channelmonitor.rs:351:2
    |
351 |       Watchtower {
    |  _____^
352 | |         revocation_base_key: PublicKey,
353 | |         htlc_base_key: PublicKey,
354 | |     }
    | |_____^

warning: unused import: `lightning::ln::peer_handler::SocketDescriptor as LnSocketTrait`
 --> /private/tmp/rust-lightning/lightning-net-tokio/src/lib.rs:9:5
  |
9 | use lightning::ln::peer_handler::SocketDescriptor as LnSocketTrait;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused `std::result::Result` that must be used
   --> /private/tmp/rust-lightning/lightning-net-tokio/src/lib.rs:174:4
    |
174 |             tokio::spawn(Self::schedule_read(peer_manager, us, reader, receiver)).await;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_must_use)]` on by default
    = note: this `Result` may be an `Err` variant, which should be handled

warning: unused `std::result::Result` that must be used
   --> /private/tmp/rust-lightning/lightning-net-tokio/src/lib.rs:195:4
    |
195 |             tokio::spawn(Self::schedule_read(peer_manager, us, reader, receiver)).await;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this `Result` may be an `Err` variant, which should be handled

   Compiling rust-lightning-bitcoinrpc v0.0.1 (/private/tmp/rust-lightning-bitcoinrpc)
error[E0277]: `std::sync::MutexGuard<'_, lightning_net_tokio::Connection>` cannot be sent between threads safely
   --> src/main.rs:521:30
    |
521 |                                             join_handles.push(tokio::spawn(async move {
    |                                                               ^^^^^^^^^^^^ `std::sync::MutexGuard<'_, lightning_net_tokio::Connection>` cannot be sent between threads safely
    | 
   ::: /Users/felixklock/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.11/src/task/spawn.rs:123:21
    |
123 |         T: Future + Send + 'static,
    |                     ---- required by this bound in `tokio::task::spawn::spawn`
    |
    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, lightning_net_tokio::Connection>`
    = note: required because it appears within the type `for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9> {std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, tokio::sync::mpsc::bounded::Sender<()>, secp256k1::key::PublicKey, tokio::net::tcp::stream::TcpStream, tokio::io::split::ReadHalf<tokio::net::tcp::stream::TcpStream>, tokio::sync::mpsc::bounded::Receiver<()>, std::sync::Arc<std::sync::Mutex<lightning_net_tokio::Connection>>, &'r lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>, &'s std::sync::Arc<std::sync::Mutex<lightning_net_tokio::Connection>>, lightning_net_tokio::SocketDescriptor, std::result::Result<std::vec::Vec<u8>, lightning::ln::peer_handler::PeerHandleError>, std::vec::Vec<u8>, &'t0 std::sync::Mutex<lightning_net_tokio::Connection>, std::result::Result<std::sync::MutexGuard<'t1, lightning_net_tokio::Connection>, std::sync::PoisonError<std::sync::MutexGuard<'t2, lightning_net_tokio::Connection>>>, lightning_net_tokio::Connection, std::sync::MutexGuard<'t3, lightning_net_tokio::Connection>, &'t4 mut std::option::Option<tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, std::option::Option<tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, std::option::Option<&'t5 mut tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, &'t6 mut tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>, &'t7 [u8], &'t8 std::vec::Vec<u8>, tokio::io::util::write_all::WriteAll<'t9, tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, (), impl std::future::Future, tokio::task::join::JoinHandle<()>}`
    = note: required because it appears within the type `[static generator@DefId(75:54 ~ lightning_net_tokio[f695]::{{impl}}[0]::setup_outbound[0]::{{closure}}[0]) 0:std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, 1:tokio::sync::mpsc::bounded::Sender<()>, 2:secp256k1::key::PublicKey, 3:tokio::net::tcp::stream::TcpStream for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9> {std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, tokio::sync::mpsc::bounded::Sender<()>, secp256k1::key::PublicKey, tokio::net::tcp::stream::TcpStream, tokio::io::split::ReadHalf<tokio::net::tcp::stream::TcpStream>, tokio::sync::mpsc::bounded::Receiver<()>, std::sync::Arc<std::sync::Mutex<lightning_net_tokio::Connection>>, &'r lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>, &'s std::sync::Arc<std::sync::Mutex<lightning_net_tokio::Connection>>, lightning_net_tokio::SocketDescriptor, std::result::Result<std::vec::Vec<u8>, lightning::ln::peer_handler::PeerHandleError>, std::vec::Vec<u8>, &'t0 std::sync::Mutex<lightning_net_tokio::Connection>, std::result::Result<std::sync::MutexGuard<'t1, lightning_net_tokio::Connection>, std::sync::PoisonError<std::sync::MutexGuard<'t2, lightning_net_tokio::Connection>>>, lightning_net_tokio::Connection, std::sync::MutexGuard<'t3, lightning_net_tokio::Connection>, &'t4 mut std::option::Option<tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, std::option::Option<tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, std::option::Option<&'t5 mut tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, &'t6 mut tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>, &'t7 [u8], &'t8 std::vec::Vec<u8>, tokio::io::util::write_all::WriteAll<'t9, tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, (), impl std::future::Future, tokio::task::join::JoinHandle<()>}]`
    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(75:54 ~ lightning_net_tokio[f695]::{{impl}}[0]::setup_outbound[0]::{{closure}}[0]) 0:std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, 1:tokio::sync::mpsc::bounded::Sender<()>, 2:secp256k1::key::PublicKey, 3:tokio::net::tcp::stream::TcpStream for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9> {std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, tokio::sync::mpsc::bounded::Sender<()>, secp256k1::key::PublicKey, tokio::net::tcp::stream::TcpStream, tokio::io::split::ReadHalf<tokio::net::tcp::stream::TcpStream>, tokio::sync::mpsc::bounded::Receiver<()>, std::sync::Arc<std::sync::Mutex<lightning_net_tokio::Connection>>, &'r lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>, &'s std::sync::Arc<std::sync::Mutex<lightning_net_tokio::Connection>>, lightning_net_tokio::SocketDescriptor, std::result::Result<std::vec::Vec<u8>, lightning::ln::peer_handler::PeerHandleError>, std::vec::Vec<u8>, &'t0 std::sync::Mutex<lightning_net_tokio::Connection>, std::result::Result<std::sync::MutexGuard<'t1, lightning_net_tokio::Connection>, std::sync::PoisonError<std::sync::MutexGuard<'t2, lightning_net_tokio::Connection>>>, lightning_net_tokio::Connection, std::sync::MutexGuard<'t3, lightning_net_tokio::Connection>, &'t4 mut std::option::Option<tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, std::option::Option<tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, std::option::Option<&'t5 mut tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, &'t6 mut tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>, &'t7 [u8], &'t8 std::vec::Vec<u8>, tokio::io::util::write_all::WriteAll<'t9, tokio::io::split::WriteHalf<tokio::net::tcp::stream::TcpStream>>, (), impl std::future::Future, tokio::task::join::JoinHandle<()>}]>`
    = note: required because it appears within the type `impl std::future::Future`
    = note: required because it appears within the type `impl std::future::Future`
    = note: required because it appears within the type `{std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, tokio::sync::mpsc::bounded::Sender<()>, secp256k1::key::PublicKey, std::net::TcpStream, std::result::Result<tokio::net::tcp::stream::TcpStream, std::io::Error>, tokio::net::tcp::stream::TcpStream, impl std::future::Future, ()}`
    = note: required because it appears within the type `[static generator@src/main.rs:521:54: 524:13 peer_manager:std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, event_notify:tokio::sync::mpsc::bounded::Sender<()>, pk:secp256k1::key::PublicKey, stream:std::net::TcpStream {std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, tokio::sync::mpsc::bounded::Sender<()>, secp256k1::key::PublicKey, std::net::TcpStream, std::result::Result<tokio::net::tcp::stream::TcpStream, std::io::Error>, tokio::net::tcp::stream::TcpStream, impl std::future::Future, ()}]`
    = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:521:54: 524:13 peer_manager:std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, event_notify:tokio::sync::mpsc::bounded::Sender<()>, pk:secp256k1::key::PublicKey, stream:std::net::TcpStream {std::sync::Arc<lightning::ln::peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, std::sync::Arc<lightning::ln::channelmanager::ChannelManager<lightning::chain::keysinterface::InMemoryChannelKeys, std::sync::Arc<ChannelMonitor>>>>>, tokio::sync::mpsc::bounded::Sender<()>, secp256k1::key::PublicKey, std::net::TcpStream, std::result::Result<tokio::net::tcp::stream::TcpStream, std::io::Error>, tokio::net::tcp::stream::TcpStream, impl std::future::Future, ()}]>`
    = note: required because it appears within the type `impl std::future::Future`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rust-lightning-bitcoinrpc`.

To learn more, run the command again with --verbose.
