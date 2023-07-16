rust
struct Client { key: String, version: u8 }

fn main() {
    let clients: &mut [Client] = &mut [];

    // Error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
    // clients.sort_by_key(|c| &c.key);

    // OK
    slice_sort_by_key::<HKT!(&str), _, _>(clients, |c| &c.key);

    // Important: owned case works too!
    slice_sort_by_key::<HKT!(u8), _, _>(clients, |c| c.version);
}
