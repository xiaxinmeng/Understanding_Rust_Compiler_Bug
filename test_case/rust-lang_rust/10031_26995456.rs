
➜ cat mod.rs
#[link(name = "min", vers = "0.1")];

pub struct Wrap<A>(A);
➜ cat test.rs
extern mod min;

#[test]
fn min_test() {
  let x: min::Wrap<int> = min::Wrap(10);
}
➜ rustc --lib mod.rs
warning: no debug symbols in executable (-arch x86_64)
➜ rustc --test -L . test.rs
test.rs:5:36: 5:38 error: mismatched types: expected `BUG[0u]` but found `<VI0>` (expected type parameter but found integral variable)
test.rs:5   let x: min::Wrap<int> = min::Wrap(10);
                                              ^~
test.rs:5:26: 5:40 error: mismatched types: expected `min::Wrap<int>` but found `min::Wrap<BUG[0u]>` (expected int but found type parameter)
test.rs:5   let x: min::Wrap<int> = min::Wrap(10);
                                    ^~~~~~~~~~~~~~
error: aborting due to 2 previous errors
task '<unnamed>' failed at 'explicit failure', /Users/utkarsh/dev/git/rust/src/libsyntax/diagnostic.rs:98
task '<unnamed>' failed at 'explicit failure', /Users/utkarsh/dev/git/rust/src/librustc/rustc.rs:395
