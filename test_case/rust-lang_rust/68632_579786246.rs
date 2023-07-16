rust
pub fn max_subarray_bad(arr: &[i32]) -> (usize, usize, i32)
{
    let zro: i32 = 0; // substitute let with const for slow version and vice versa
    let prefixes = arr
        .iter()
        .enumerate()
        .scan((0, 0), |s, (i, v)| {
            if s.1 > zro {
                s.1 = s.1 + *v;
            } else {
                *s = (i, *v);
            }
            Some(*s)
        });
    let (right_idx, (left_idx, sum)) = prefixes
        .enumerate()
        .max_by_key(|&(_, (_, sum))| sum)
        .unwrap();

    (left_idx, right_idx + 1, sum)
}

fn main() {
    const N: usize = 1000000;
    let v = vec![0; N];
    const loops: usize = 1000;
    let mut total_sum = 0;
    for i in 0..loops {
        let (_, _, sum) = max_subarray_bad(&v);
        total_sum += sum;
    }
    println!("{}", total_sum);
}
