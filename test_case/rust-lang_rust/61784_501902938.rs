
macro_rules! rotate {
    ($fn:ident, $n:expr, $mapper:expr) => {
        #[bench]
        fn $fn(b: &mut Bencher) {
            let mut x = (0usize..$n).map(&$mapper).collect::<Vec<_>>();
            b.iter(|| {
                for s in 0..x.len() {
                    x[..].rotate_right(s);
                }
                black_box(x[0].clone())
            })
        }
    }
}

#[derive(Clone)]
struct Rgb(u8,u8,u8);

rotate!(rotate_u8, 32, |i| i as u8);
rotate!(rotate_rgb, 32, |i| Rgb(i as u8, (i as u8).wrapping_add(7), (i as u8).wrapping_add(42)));
rotate!(rotate_usize, 32, |i| i);
rotate!(rotate_usize_4, 32, |i| [i; 4]);
rotate!(rotate_usize_32, 32, |i| [i; 32]);
rotate!(rotate_usize_33, 32, |i| [i; 33]);
