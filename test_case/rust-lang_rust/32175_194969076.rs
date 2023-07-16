 rust
#![feature(question_mark, type_ascription)]

fn main() {
    (42: Result<_,_>)?;
}
