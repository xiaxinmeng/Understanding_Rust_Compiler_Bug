 rust
extern crate test;
use test::Bencher;

fn n_queens(n: i32) -> uint {
    return n_queens_helper((1 << n as uint) -1, 0, 0, 0);
}

fn n_queens_helper(all_ones: i32, left_diags: i32, columns: i32, right_diags: i32) -> uint {
    let mut solutions = 0;
    let mut valid_spots = !(left_diags | columns | right_diags) & all_ones;
    while valid_spots != 0 {
        let spot = -valid_spots & valid_spots;
        valid_spots = valid_spots ^ spot;
        solutions += n_queens_helper(
            all_ones,
            (left_diags | spot) << 1,
            (columns | spot),
            (right_diags | spot) >> 1);
    }
    return solutions + ((columns == all_ones) as uint)
}

//#[test]
//fn test_parallel_n_queens() {
//    n_queens_helper(0, 0, 0, 0);
//}

#[bench]
fn bench_n_queens(b: &mut Bencher) {
    b.iter(|| test::black_box(n_queens(12)));
}
