 rust
macro_rules! for_gud {
    (($pat:pat) in $head:expr { $($body:stmt)* }) => { {
        match IntoIterator::into_iter($head) {
            mut iter => loop {
                match Iterator::next(&mut iter) {
                    Some($pat) => { $($body)* }
                    None => break,
                }
            }
        }
    } }
}
