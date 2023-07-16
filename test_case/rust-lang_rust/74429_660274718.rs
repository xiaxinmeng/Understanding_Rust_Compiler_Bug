rust
use ndarray::{s, Array2, Axis, Zip};

pub fn gaussian_elimination<T>(a: Array2<T>)
{
    (0..1).filter(|_| true);

    let (rest_equations, this_equation) =
        a.slice(s![.., ..]).split_at(Axis(0), 0);
    let this_equation = this_equation.index_axis(Axis(0), 0);
    rest_equations.axis_iter(Axis(0)).for_each(|_| {
        Zip::from(this_equation);
    });
}
