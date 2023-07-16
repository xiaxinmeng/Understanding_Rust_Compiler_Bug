
let my_data = ~new_unwrapper_data();
if cmpxchg(&mut unwrapper_data, my_data, 0) {
    // success path
} else {
    fail ~"another task is already unwrapping"
}
