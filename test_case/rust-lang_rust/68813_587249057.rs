rust
struct RPCClient;

use lightning_net_tokio::Connection;

use tokio::sync::mpsc;
use lightning::ln::{channelmonitor, channelmanager, peer_handler};
use lightning::chain::keysinterface;

use std::sync::Arc;

struct ChannelMonitor;
impl channelmonitor::ManyChannelMonitor for ChannelMonitor {}

#[tokio::main]
async fn main() {
	let peer_manager: Arc<peer_handler::PeerManager<lightning_net_tokio::SocketDescriptor, Arc<
                channelmanager::ChannelManager<keysinterface::InMemoryChannelKeys, Arc<ChannelMonitor>>
            >>> = unimplemented!();
        let event_notify: mpsc::Sender<()> = unimplemented!();
        let stream: tokio::net::TcpStream = unimplemented!();
        let pk: secp256k1::key::PublicKey = unimplemented!();
        tokio::spawn(async move {
                Connection::setup_outbound(peer_manager, event_notify, pk, stream).await;
        });
}
