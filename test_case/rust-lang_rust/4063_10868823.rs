
enum State { ST_NULL, ST_WHITESPACE }

struct StateTable {
    entry: ~[mut ~[mut State]],
}

priv fn SomeFunction () -> StateTable {
    StateTable { entry: ~[mut ~[mut ST_NULL, ..(ST_WHITESPACE as uint)], ..128] }
}
