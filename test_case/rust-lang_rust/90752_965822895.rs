rust
struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let mut foo = None;
    match foo {
        None => (),
        _ => return,
    }

    // Here we know that `foo` is `None`, otherwise we would have returned above.

    // Overwrite `foo` indirectly. `foo` is no longer `None`, but we fail to update our
    // initialization data to reflect this.
    *(&mut foo) = Some((S(0), S(1)));

    // Move from `(foo as Some).0` (despite the fact that we think it is uninitialized),
    // creating a drop flag for `(foo as Some).0` and making the subsequent
    // drop of `foo` an open one.
    match foo {
        Some((_x, _)) => {
        } // `_x` is dropped: "dropping 0"

        _ => {}
    }

    // `foo` is dropped, but it is an "open" drop, so the move paths associated with its
    //`Some` variants are still marked as uninitialized and the `Drop` is eliminated.
}
