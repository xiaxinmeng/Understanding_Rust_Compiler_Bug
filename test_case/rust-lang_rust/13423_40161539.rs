
for stream in acceptor.incoming() {
     match stream {
          Err(_) => continue,
          Ok(socket) => spawn(proc() { handle_client(socket); }
     }
}
