rust
let server = listener
    .incoming()
    .map_err(|e| eprintln!("accept failed = {:?}", e))
    .for_each(move |sock| {
        let (reader, writer) = sock.split();
        let req: Vec<u8> = Vec::new();
        let task = io::read_exact(reader, req)
            .and_then(|(_res, rq)| {
                ...
                io::write_all(writer, RESPONSE_PREAMBLE)
            })
            .map(|(w, _e)| {
                ...
                let jpg: Box<[u8]> = wr.into_inner().unwrap().into_boxed_slice();
                io::write_all(w, jpg)
            })
            .then(|_| Ok(()));
        tokio::executor::spawn(task)
});
