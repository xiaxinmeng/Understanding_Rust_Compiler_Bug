
trait vec_like<T> {
    req fn get(idx: uint) -> T;
    req fn set(idx: uint, T value);
    req fn len() -> uint;

    // foreach works great
    fn foreach(b: block(T) => ()) {
        let i = 0u, n = len();
        while i < n { b(get(i)); }
    }

    // map... not so great. what is the return type?
    fn map<S>(b: block(T) => S) -> ??? {
    }
}
