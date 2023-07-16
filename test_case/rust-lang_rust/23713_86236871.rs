 rust
fn test_05() {
    let mut item = Item { id: 1 };
    let entry_ref = &mut EntryMutBorrow { body: Some(&mut item) };

    // The following type checks, so I guess the type of `entry_ref.body`
    // is `Option<&mut Item>` here. The program is stil rejected because
    //
    //   error: cannot move out of borrowed content
    //
    let body_ref: Option<&mut Item> = entry_ref.body;
}
