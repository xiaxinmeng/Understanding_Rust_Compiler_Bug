rust
panic::update_hook(|prev, info| {
    println!("panic handler A");
    prev(info);
});
