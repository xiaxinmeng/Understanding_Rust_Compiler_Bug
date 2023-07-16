 rust
struct Event {
    id: i32,
    sender: ~str,
}

fn read_event<R: Reader>(r: &mut R) -> IoResult<Event> {
    let id = if_ok!(r.read_be_i32());
    let sender_len = if_ok!(r.read_be_u32());
    let sender = if_ok!(r.read_bytes(sender_len as uint));
    Ok(Event {
        id: id,
        sender: str::from_utf8_owned(sender),
    })
}
