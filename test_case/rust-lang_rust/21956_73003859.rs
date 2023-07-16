 rust
fn main() {
    let mut session = Session{subscriptions : HashMap::new()};
    {
        let sub_id = session.subscription().create();
    }
    session.do_nothing();
}
