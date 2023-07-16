rust
fn main() {
    let nums = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    // Any number is ok, but must be i32.
    let insert_num = 14;
    // Find the index
    let index = match nums.binary_search(&insert_num) {
        Ok(a) => a,
        Err(b) => b
    };
    let mut insert_nums: [i32; 14] = [0; 14];
    // Insert into a new array.
    for i in 0..14 {
        if i > index {
            insert_nums[i] = nums[i - 1];
        } else if i < index {
            insert_nums[i] = nums[i];
        } else {
            insert_nums[i] = insert_num;
        }
    }
    println!("{:?}", insert_nums);
    assert_eq!([-100, 0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55], insert_nums);
}
