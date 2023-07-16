
fn user(kv: &Store) {
    let mut tx = Transaction { kv, reads: vec![] };
    tx.scan().for_each(|x| println!("{:?}", x));
    tx.scan().for_each(|x| println!("{:?}", x));
}
