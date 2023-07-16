
fn foo<T: Default>(list: &mut Vec<T>) {
    let mut cloned_items = Vec::new();
    for v in list.iter() {
        cloned_items.push(v.clone())
    }
    list.push(T::default());
    // Use `cloned_items` to defeat NLL
    drop(cloned_items);
}
