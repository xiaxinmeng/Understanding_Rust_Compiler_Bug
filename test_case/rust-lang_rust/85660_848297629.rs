rust
fn copy_to_front_evens2(arr: &mut [u32; 100]) {
    let mut pos = 0; // Tortoise
    for i in 0 .. arr.len() { // Hare
        if arr[i] % 2 == 1 {
            pos = pos.min(arr.len() - 1);
            arr[pos] = arr[i];
            pos += 1;
        }
    }
}
