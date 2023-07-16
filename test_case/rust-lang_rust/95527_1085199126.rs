rust
fn bar<'at_x, 'x> (at_x: &'at_x &'x u8)
{
    ::std::thread::scope::<'at_x>(move |s| {
        s.spawn(move || drop(*at_x));
    });
}
