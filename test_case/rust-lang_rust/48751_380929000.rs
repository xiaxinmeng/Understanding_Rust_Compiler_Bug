rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FromResolver {
  Resolved(Vec<SocketAddr>),
  Response(Vec<Path>),
  Published,
  Unpublished,
  Error(String)
}

fn send(
  w: WriteHalf<TcpStream>,
  m: FromResolver
) -> impl Future<Item=WriteHalf<TcpStream>, Error=()> {
  match serde_json::to_vec(&m) {
    Err(_) => future::err(()),
    Ok(m) => future::ok(m)
  }.and_then(|m| {
    write_all(w, m).map_err(|_| ())
      .and_then(|(w, _)| {
        write_all(w, "\n").map_err(|_| ())
          .and_then(|(w, _)| Ok(w))
      })
  })
}
