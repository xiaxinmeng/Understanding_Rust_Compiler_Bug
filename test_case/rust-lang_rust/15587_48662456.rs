 rust
macro_rules! vec{
    ($e:expr, ..$n:expr) => { ::std::vec::Vec::from_elem($n, $e) };
    ($($e:expr),*) => {{
        let mut _temp = ::std::vec::Vec::new();
        $(_temp.push($e);)*
        _temp
    }};
    ($($e:expr),+,) => (vec!($($e),+));
}
