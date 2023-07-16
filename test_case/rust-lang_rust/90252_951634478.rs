rust
fn some_fn() {
    let mut widths = vec![(0usize, 0usize); 100];
    let mut stack = vec![0usize];  // <---- 

    let a = stack.pop().unwrap();
    let idx = 0usize;
    widths[a].1 = idx;

    stack.push(idx);
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut widths = vec![(0usize, heights.len()); heights.len()];
    let mut stack = vec![0usize]; // <---- 

    for (idx, &x) in heights.iter().enumerate() {
        while let Some(&pos) = stack.last() {
            if x >= heights[pos] {
                break;
            }

            widths[pos].1 = idx;
            stack.pop();
        }

        stack.push(idx);
    }

    todo!()
}
