
    #[bench]
    fn bench_vec_iter_max(b: &mut Bencher) {
        let n = 5;
        let v: Vec<i32> = (0..n).map(|x| x as i32).collect();
        b.iter(|| v.iter().max())
    } 

    #[bench]
    fn bench_vec_max(b: &mut Bencher) {
        let n = 5;
        let v: Vec<i32> = (0..n).map(|x| x as i32).collect();
        b.iter(|| vec_max(&v))
    }

#[inline]
fn vec_max<T: Ord + Copy>(v: &[T]) -> Option<T> {
    if v.is_empty() {
        return None;
    }
    let mut max_val = v[0];
    if v.len() == 1 {
        return Some(max_val);
    }
    for val in &v[1..] {
        if *val > max_val {
            max_val = *val;
        }
    }
    Some(max_val)
}
