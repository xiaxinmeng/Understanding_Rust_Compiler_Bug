 rust
macro_rules! for_ok {
    (($pat:pat) in $head:expr { $($body:stmt)* }) => { {
        let head = $head;
        let mut iter = IntoIterator::into_iter(head);
        loop {
            match Iterator::next(&mut iter) {
                Some($pat) => { $($body)* }
                None => break,
            }
        }
    } }
}
