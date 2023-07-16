
#![feature(staged_api)]
#![staged_api]

#[unstable(feature = "lol", issue = "1")]
#[deprecated(since = "1.0.0", reason = "lol")]
mod nope {
    pub fn foo() {}
    pub fn bar() {}
}

fn main() {
    nope::foo();
    nope::bar();
}
