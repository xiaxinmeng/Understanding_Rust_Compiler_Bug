 rust
macro_rules! for_bad {
    (($pat:pat) in $head:expr { $($body:stmt)* }) => { {
        let mut iter = IntoIterator::into_iter($head);
        loop {
            match Iterator::next(&mut iter) {
                Some($pat) => { $($body)* }
                None => break,
            }
        }
    } }
}
