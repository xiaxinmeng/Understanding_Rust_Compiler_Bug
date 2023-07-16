rust
use ndarray::{Array2, Axis, Zip};

fn x<T>(a: Array2<T>) {
    (0..1).filter(|_| true);
    let y = a.index_axis(Axis(0), 0);
    a.axis_iter(Axis(0)).for_each(|_| {
        Zip::from(y);
    });
}
