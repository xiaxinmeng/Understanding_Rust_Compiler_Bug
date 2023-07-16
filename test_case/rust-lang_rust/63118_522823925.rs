rust
fn main() {
    let array: Box<[u8; 4]> = Box::new([1, 2, 3, 4]);

    match array {
        nums
//      ---- `nums` is bound by move.
            if nums.iter().sum::<u8>() == 10
//                 ^------ `.iter()` implicitly takes a reference to `nums`.
        => {
            drop(nums);
//          --------- Legal as `nums` was bound by move and so we have ownership.
        }
        _ => unreachable!(),
    }
}
