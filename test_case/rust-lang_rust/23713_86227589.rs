 rust
struct Item {
    id: isize
}

struct EntryMutBorrow<'a> {
    body: Option<&'a mut Item>
}

fn main() {
    let mut item = Item { id: 1 };
    let entry_ref = &mut EntryMutBorrow { body: Some(&mut item) };

    // ATTEMPT FAILED:
    // The following does not work as moving a value from a borrow
    // is not possible and `entry_ref.body` a borrow.
    //
    //   error: cannot move out of borrowed content
    //
    // let item_mut_borrow: &mut Item = entry_ref.body.unwrap();

    // ATTEMPT WORKS:
    // The following works, but it is not clear to me why: Why is moving the
    // value from an `Option<&mut &mut Item>` type not an problem but having
    // a single borrow as `Option<&mut Item>` is not enough?
    let item_as_ref: Option<&mut &mut Item> = entry_ref.body.as_mut();
    let item_mut_borrow: &mut &mut Item = item_as_ref.unwrap();

    // Update the id on the entry.item.id - the println! confirms the change succeeded.
    item_mut_borrow.id = 2;
    println!("entry.body.id={}", item_mut_borrow.id);

    // PROGRAM OUTPUT:
    // entry.body.id=2
}
