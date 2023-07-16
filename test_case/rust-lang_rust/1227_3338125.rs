
iface seq<T> {
    fn len() -> uint;
    fn iter(block(T));
}
