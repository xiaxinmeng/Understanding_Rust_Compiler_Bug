 rust
struct Vec2<S>(S, S);
fn from_array(arr: [S, .. 2]) -> Vec<S> {
    unsafe {
        let x = Vec2(ptr::read(&arr[0]), ptr::read(&arr[1]));
        // stop the destructor running on the values in the array, i.e. transfer ownership:
        mem::forget(arr); 
        x
    }
}
