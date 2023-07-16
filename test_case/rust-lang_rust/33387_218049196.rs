 shell
$ RUST_BACKTRACE=1 cargo build --release -p ethcore
   Compiling ethcore v1.2.0 (file:///parity-master/ethcore)
ethcore/src/trace/db.rs:25:12: 25:21 warning: unused import, #[warn(unused_imports)] on by default
ethcore/src/trace/db.rs:25 use util::{FixedHash, H256, H264, Database, DBTransaction};
                                      ^~~~~~~~~
error: internal compiler error: src/librustc_trans/common.rs:1102: Encountered error `Unimplemented` selecting `Binder(<util::network::host::Host<service::SyncMessage> as util::IoHandler<util::NetworkIoMessage<Message>>>)` during trans
Segmentation fault
error: Could not compile `ethcore`.
