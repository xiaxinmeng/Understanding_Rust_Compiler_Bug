rust
fn remove_last_node_iterative<T>(node_ref: &mut List<T>) {
    let mut current_ref = &mut *node_ref;
    
    loop {
        let next_ref = &mut current_ref.as_mut().unwrap().next;

        if next_ref.is_some() {
            current_ref = next_ref;
        } else {
            break;
        }
    }

    *node_ref = None; // This line causes lifetime error.
}
