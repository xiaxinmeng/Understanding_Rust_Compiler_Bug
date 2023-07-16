
error[E0382]: borrow of moved value: `x`
    --> src/librustc_mir/hair/pattern/_match.rs:1614:53
     |
1611 |                         if let Some(range) = x {
     |                                     ----- value moved here
...
1614 |                         debug!("intersection {:?}", x);
     |                                                     ^ value borrowed here after partial move
     |
     = note: move occurs because value has type `hair::pattern::_match::IntRange<'_>`, which does not implement the `Copy` trait
       Consider borrowing `x` rather than moving it.
