
trait vec_like<T> {
    ...
    fn map<S>(b: block(T) => S) -> [S] {
        let res = [];
        foreach { |e| res += [e]; }
        return e;
    }
}
