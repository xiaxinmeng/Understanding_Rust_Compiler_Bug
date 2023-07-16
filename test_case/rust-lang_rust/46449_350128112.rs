rust
#![forbid(warnings)]

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

mod config {
    use std::net::SocketAddr;
    pub struct ServerAddr;

    impl ServerAddr {
        pub fn listen_addr(&self) -> &SocketAddr {
            loop {}
        }
    }
    pub struct ServerConfig;
    impl ServerConfig {
        pub fn addr(&self) -> &ServerAddr {
            loop {}
        }
    }
    pub struct Config {
        pub server: Option<ServerConfig>,
    }
}
pub mod relay {
    use std::io;
    use config::Config;
    use futures::Future;
    use tokio_core::reactor::Handle;
    pub mod tcprelay {
        use std::io;
        use relay::{boxed_future, BoxIoFuture};
        use futures::{Future};
        pub mod server {
            use std::io;
            use relay::BoxIoFuture;
            use relay::Context;
            use futures::Future;
            use futures::stream::Stream;
            use tokio_core::net::{TcpListener};
            use super::{tunnel, EncryptedHalfFut};
            use super::utils::CopyTimeoutOpt;

            struct TcpRelayClientHandshake;
            impl TcpRelayClientHandshake {
                fn handshake(self) -> BoxIoFuture<TcpRelayClientPending> {
                    loop {}
                }
            }

            struct TcpRelayClientPending;
            impl TcpRelayClientPending {
                fn connect(self) -> BoxIoFuture<TcpRelayClientConnected> {
                    loop {}
                }
            }

            struct TcpRelayClientConnected;
            fn copy_timeout_opt() -> CopyTimeoutOpt {
                loop {}
            }
            fn client() -> EncryptedHalfFut {
                loop {}
            }

            impl TcpRelayClientConnected {
                fn tunnel(self) -> BoxIoFuture<()> {
                    tunnel(copy_timeout_opt(), client())
                }
            }

            pub fn run() -> Box<Future<Item = (), Error = io::Error>> {
                let mut fut: Option<Box<Future<Item = (), Error = io::Error>>> = None;
                Context::with(|ctx| {
                    let config = ctx.config();
                    for svr_cfg in &config.server {
                        let listener = {
                            let addr = svr_cfg.addr();
                            let addr = addr.listen_addr();
                            let listener =
                                TcpListener::bind(&addr, ctx.handle()).unwrap_or_else(|_| loop {});
                            listener
                        };
                        let listening = listener.incoming().for_each(move |_| {
                            let client = TcpRelayClientHandshake {};
                            let fut = client
                                .handshake()
                                .and_then(|c| c.connect())
                                .and_then(|c| c.tunnel())
                                .map_err(|_| ());
                            Context::with(|ctx| ctx.handle().spawn(fut));
                            Ok(())
                        });
                        fut = Some(Box::new(listening)
                            as Box<Future<Item = (), Error = io::Error>>)
                    }
                    loop {}
                })
            }
        }
        mod utils {
            use std::io;
            use futures::{Future, Poll};

            pub struct CopyTimeoutOpt {
                _buf: [u8; 8192],
            }
            impl Future for CopyTimeoutOpt {
                type Item = (u64, (), ());
                type Error = io::Error;
                fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                    loop {}
                }
            }
        }

        struct EncryptedHalf;

        type EncryptedHalfFut = BoxIoFuture<EncryptedHalf>;

        fn tunnel<CF, CFI, SF, SFI>(c2s: CF, s2c: SF) -> BoxIoFuture<()>
        where
            CF: Future<Item = CFI, Error = io::Error> + 'static,
            SF: Future<Item = SFI, Error = io::Error> + 'static,
        {
            let c2s = c2s.then(move |res| match res {
                Ok(..) => Ok(()),
                Err(err) => Err(err),
            });
            let s2c = s2c.then(move |res| match res {
                Ok(..) => Ok(()),
                Err(err) => Err(err),
            });
            let fut = c2s.select(s2c)
                .and_then(move |_| Ok(()))
                .map_err(|(err, _)| err);
            boxed_future(fut)
        }
    }

    type BoxIoFuture<T> = Box<Future<Item = T, Error = io::Error>>;
    fn boxed_future<T, E, F>(f: F) -> Box<Future<Item = T, Error = E>>
    where
        F: Future<Item = T, Error = E> + 'static,
    {
        Box::new(f)
    }
    thread_local!(static CONTEXT: Context = loop {});
    struct Context;
    impl Context {
        fn with<F, R>(f: F) -> R
        where
            F: FnOnce(&Context) -> R,
        {
            CONTEXT.with(f)
        }

        fn handle(&self) -> &Handle {
            loop {}
        }

        fn config(&self) -> &Config {
            loop {}
        }
    }
}
