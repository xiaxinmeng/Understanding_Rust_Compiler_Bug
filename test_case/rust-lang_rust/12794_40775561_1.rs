 rs
impl<D:Decoder<E>, E> Decodable<D, E> for LogEntry {
    fn decode(d: &mut D) -> Result<LogEntry, E> {
        d.read_struct("LogEntry", 4u, |d|
            Ok(LogEntry {
                index: try!(d.read_struct_field("index", 0u, |d| Decodable::decode(d))),
                term: try!(d.read_struct_field("term", 1u, |d| Decodable::decode(d))),
                command_name: try!(d.read_struct_field("command_name", 2u, |d| Decodable::decode(d))),
                command: match d.read_struct_field("command", 3u, |d| Decodable::decode(d))) {
                    Ok(opt) => opt,
                    Err(_) => None
                }
            })
        )
    }
}
