rust
struct Id {}
struct E { id: Id }
struct A {}

fn response(id: Id) -> E { E { id } }

fn send<'a>(_: &'a A, _: E) {}

fn foo() {
    let evt = E { id: Id {} };
    match evt {
        ref resp if true => {}
        ref resp if true => {
            let tx = A {};
            send(&tx, response(resp.id));
        }
        _ => (),
    }
}
