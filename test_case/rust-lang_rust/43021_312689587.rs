rust
fn run<T>(socket: TcpStreamNew, test: &T)
         -> impl Future<Item=Status, Error=Error>
where T: Test {
    let request = socket.and_then(move |socket|
        io::write_all(socket, T::request().as_bytes()));

    let response = request
        .and_then(|(socket, _req)| io::read_to_end(socket, Vec::new()) )
        .and_then(|(_socket, bytes)| {
            let mut headers = [EMPTY_HEADER; MAX_HEADERS];
            let mut response = Response::new(&mut headers);
            future::result(
                response.parse(&bytes)
                    .map(|_| response)
                    .map_err(|e| Error::new(ErrorKind::Other, e)))
        });
    let status = response.map(T::check);
    status
}
