
steve@warmachine:~/tmp$ cat hello.rs
fn main() {
    /* 42 */
}
steve@warmachine:~/tmp$ rustc hello.rs --pretty=expanded -Z unstable-options
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
fn main() {
    /* 42 */
}
steve@warmachine:~/tmp$ rustc hello.rs --pretty=identified -Z unstable-options
fn main() {
    /* 42 */
} /* block 4294967295 */ /* 4294967295 */

