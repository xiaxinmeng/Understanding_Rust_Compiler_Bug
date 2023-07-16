
type Connection = fn@(~[u8]);

fn f() -> option<Connection> {
    let mock_connection: Connection = fn@(data: ~[u8]) { };
    some(mock_connection)
}

fn main() {
}
