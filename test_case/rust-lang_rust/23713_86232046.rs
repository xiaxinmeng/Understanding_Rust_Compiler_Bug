 rust
struct Item {
    id: isize
}

struct EntryMutBorrow<'a> {
    body: Option<&'a mut Item>
}

fn main() {
    let mut item = Item { id: 1 };
    let entry_ref = EntryMutBorrow { body: Some(&mut item) };
    let item_mut_borrow: &mut Item = entry_ref.body.unwrap();
    item_mut_borrow.id = 2;
    println!("entry.body.id={}", item_mut_borrow.id);

    // PROGRAM OUTPUT:
    // entry.body.id=2
}
