
let my_data = ~new_unwrapper_data();
loop {
    let oldval = load_linked(&mut unwrap_data);
    if oldval != 0 {
        fail ~"another task is already unwrapping"
    }
    if store_conditional(&mut unwrap_data, my_data) {
        // success path
    }
    // got interrupted / swapped out? keep trying
}
