
fn read_all_messages(stream: &mut BufRead) -> io::Result<Vec<Message>> {
    let mut r = Vec::new();
    while !stream.is_eof()? {
        r.push(read_one_message(stream)?);
    }
    r
}
