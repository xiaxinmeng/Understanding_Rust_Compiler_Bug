rust
#![feature(lint_reasons)]
#![expect(unused_variables)]

pub fn f() {
    #![warn(unused_variables)]
    let x = 1;
}
