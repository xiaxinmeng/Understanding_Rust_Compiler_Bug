 rust
// Do things up here to setup processor tasks.

let request_processor_ports = [...];
let socket = bind_listen_and_accept();

while true {
    let selected = select(socket, request_processor_ports);

    if (selected == socket) {
        available_request_processor.process(socket.read());
    } else {
        let msg = selected.read();
        socket.write(msg);
    }
}
