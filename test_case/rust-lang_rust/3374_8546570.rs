
use std;

fn main()
{
    let a = ~[1, 4, 3, 5, 2];
    let mut compare_count = 0;

    std::sort::quick_sort(|x, y| { compare_count += 1; *x < *y }, a);

    error!("%d comparisons", compare_count);
    error!("%?", a);
}
