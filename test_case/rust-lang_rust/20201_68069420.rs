
$ cargo test
…
/home/nagisa/files/rust/fails/water.rs/target/safesync-490939a7cdc50d58.o: In function `message::SyncMessage::get_payload::h8503016970478518134':
safesync.0.rs:(.text._ZN7message11SyncMessage11get_payload20h8503016970478518134E+0x17b): undefined reference to `message::SyncMessage::get_payload::_FILE_LINE::h7cd42812b01f00a3Vzb'
/home/nagisa/files/rust/fails/water.rs/target/safesync-490939a7cdc50d58.o: In function `message::RawMessage::readstructunsafe::h9415964264797449173':
safesync.0.rs:(.text._ZN7message10RawMessage16readstructunsafe20h9415964264797449173E+0xa8): undefined reference to `message::RawMessage::readstructunsafe::_FILE_LINE::h7cd42812b01f00a3Ilb'
collect2: error: ld returned 1 exit status
