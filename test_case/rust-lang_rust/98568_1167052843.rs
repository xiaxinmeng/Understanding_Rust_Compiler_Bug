rust
pub fn buggy(arr: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut cnt = 0;
    // The entire loop is optimized away in release: https://godbolt.org/z/j538GxzT1
    for d in arr {
        if d > 0 {
            if prev < 0 {
                cnt += 1;
            } else {
                cnt = 1;
            }
        } else {
            if prev > 0 {
                cnt += 1;
            } else {
                cnt = 1;
            }
        }
        prev = d;
    }
    cnt
}

fn main() {
    let v = vec![-1,1];
    let ans = buggy(v);
    // The right answer is 2. But when build with release, the output is 1
    println!("{}", ans);
}
