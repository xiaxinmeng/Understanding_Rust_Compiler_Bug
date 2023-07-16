rust
const ARRAY: [i32; 3] = [0; 3];
    ARRAY[1] = 2;
    for x in &ARRAY {
        print!("{} ", x);
    }
    // output: 0 0 0 % 
