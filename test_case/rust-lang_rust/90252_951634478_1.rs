rust
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut widths = vec![(0usize, heights.len()); heights.len()];
    let mut stack = vec![];

    for (idx, &x) in heights.iter().enumerate() {
        stack.push(idx); // <---- 
        while let Some(&pos) = stack.last() {
            if x >= heights[pos] {
                break;
            }

            widths[pos].1 = idx;
            stack.pop();
        }
    }

    todo!()
}
