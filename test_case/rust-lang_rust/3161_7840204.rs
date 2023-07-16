
#[test]
fn crash() {
    struct X {
        x: ~~int;
        drop {
            error!("freeing %d", **self.x);
        }
    }
    let x = X { x: ~~5 };
    util::ignore(x);
}
