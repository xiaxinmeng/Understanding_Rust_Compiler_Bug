rust
#[rustc_layout(debug)]
type Tea<'a> = <&'a [u32] as IntoIterator>::IntoIter;
