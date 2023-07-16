rust
use proptest::{prelude::*, test_runner::TestError};
fn gen_all(_len: usize) -> impl Strategy<Value = ()> {
    Strategy::prop_map(0..=0, move |_| ())
}

fn foo(mut runner: proptest::test_runner::TestRunner) -> Result<(), TestError<()>> {
    runner
    .run(
        &::proptest::strategy::Strategy::prop_map(
            (1usize..=1).prop_flat_map(gen_all),
            |_| (),
        ),
     |()| Ok(()),
    )
}

pub fn main() {
    let x = foo as fn(_) -> _;
    println!("{x:p}");
}
