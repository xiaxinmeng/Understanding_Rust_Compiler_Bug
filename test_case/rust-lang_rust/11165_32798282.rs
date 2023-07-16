 rust
fn main() {
    // Create an event queue. That's the central element instead of Select
    let mut ev_queue = EventQueue::new(); 
    // Create a port/chan pair. Same as rust std API, but different in implementation
    let (port,chan): (Port<~str>, Chan<~str>) = Chan::new();
    // Upgrade the port to a selectable port. Still need another name for that
    let mut selport = SelectablePort::from_port(port, &ev_queue);

    // Create an event based timer 
    let mut main_timer = Timer::create(&ev_queue).unwrap();
    main_timer.set_interval(2000);  
    main_timer.start();

    // Start a subtask that communicates with us
    do native::task::spawn() {
        subtask(chan);
    }

    loop {
        // Wait for events to arrive
        let event = ev_queue.next_event().unwrap();         

        // Look for the origin and the type and then check what to do
        if event.originates_from(selport) {
            match event.event_type {
                event::ChannelMessageEvent => {
                    // We know that we received a message and can fetch it in a nonblocking style
                    let msg = selport.recv().unwrap();
                    println!("Message from subtask: {}", msg);
                },
                event::ChannelClosedEvent => {
                    // Oh, the channel seams dead now
                    println!("Subtask closed");
                    return;
                },
                _ => ()
            }
        }
        else if event.originates_from(main_timer) {
            // The timer send a tick event
            println!("main_timer::tick()");
        }
    }
}

fn subtask(chan: Chan<~str>) {
    // The other task also gets an event queue and a timer
    let mut ev_queue = EventQueue::new();
    let mut sub_timer = Timer::create(&ev_queue).unwrap();
    let mut iterations = 3;
    let mut stream_alive = false;

    // We will play with TCP here, so let's connect to somewhere. This is currently blocking
    let opt_ipaddr:Option<IpAddr> = FromStr::from_str("192.168.1.99");
    let socketaddr = SocketAddr {ip: opt_ipaddr.unwrap(), port: 8000};
    let mut rawstream = TcpStream::connect(socketaddr).unwrap();
    let mut stream = SelectableTcpStream::from_tcp_stream(rawstream, &ev_queue);
    stream_alive = true;

    // Start a timer
    sub_timer.set_interval(3000);   
    sub_timer.start();

    // Send a request. This is also currently blocking
    let request = ~"GET /index.html HTTP/1.1\r\n\r\n";
    stream.write(request.as_bytes());
    iterations -= 1;

    loop {
        // Fetch events and checkout what to do
        let event = ev_queue.next_event().unwrap();

        if event.originates_from(sub_timer) {
            // Send something to the mainthread.
            chan.send(~"subtimer::tick()");
            if !stream_alive {              
                if iterations > 0 {
                    iterations -= 1;
                    // Create a new stream. The old one wil be killed through RAII here
                    rawstream = TcpStream::connect(socketaddr).unwrap();
                    stream = SelectableTcpStream::from_tcp_stream(rawstream, &ev_queue);
                    stream_alive = true;
                    stream.write(request.as_bytes());
                }
                else {
                    return;                     
                }
            }
        }
        else if event.originates_from(stream) {
            match event.event_type {
                event::StreamClosedEvent => {
                    // Oops, the TCP connection was closed by the remote
                    chan.send(~"TCP connection closed");
                    stream_alive = false;
                },
                event::DataAvailableEvent(nr_bytes) => {
                    // Yay, we know that we received at least nr_bytes and can "safely" read them in a nonblocking fashion
                    let mut buffer: ~[u8] = std::vec::from_elem::<u8>(nr_bytes, 0);
                    let read_res = stream.read(buffer);
                    match read_res {
                        Err(err) => {
                            chan.send(err.desc.to_owned());
                        }
                        Ok(nr_read) => {
                            let txt = std::str::from_utf8(buffer.slice(0, nr_read));
                            chan.send(txt.to_owned());
                        }
                    }
                },
                _ => ()
            }
        }
    }

    // Stop my IO performing objects. This will also by done in RAII style if you forget it.
    sub_timer.stop();
    stream.close();
}
