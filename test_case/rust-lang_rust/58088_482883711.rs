rust
use tokio   :: { net::{ TcpStream }, codec::{ Framed } };
use futures :: { stream::SplitSink                     };

let mb: Inbox<MyPeer> = Inbox::new();

error: the trait bound `SplitSink<Framed<TcpStream, [...]> is not satisfied`
