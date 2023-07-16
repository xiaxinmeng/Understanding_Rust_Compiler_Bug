rust
// A common use case for me for this is:
let mut mat = [Vec<u32>; 60]::generate(|_| vec![]);

// Lately I've used:

let mut mat: [Vec<u32>; 60] = [(); 60].map(|_| vec![]);

// Two common use cases for me for collect-in-array and collect-in-slice:

let mat = lines_of_file("matrix_data.txt")
          .map(|row| row
                     .split_whitespace()
                     .map(|x| x.parse::<u32>().unwrap())
                     .collect_array::<N>())
          .collect_array::<L>();


let items = &mut [0; OVERESTIMATE];
let items = (0 .. OVERESTIMATE)
            .filter(|&i| is_good(i))
            .collect_slice_mut(items);
