rust
let mut iter = <&(dyn Error + 'static) as IntoIterator>::into_iter(&err);
