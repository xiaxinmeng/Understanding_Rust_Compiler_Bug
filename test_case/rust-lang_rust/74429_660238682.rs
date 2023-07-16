rust
use ndarray::{s, Array2, Axis, Zip};

pub fn gaussian_elimination<T>(mut a: Array2<T>)
{
    let (mut rest_equations, this_equation) =
        a.slice_mut(s![..=0, ..]).split_at(Axis(0), 0);
    let this_equation = this_equation.index_axis(Axis(0), 0);
    rest_equations.axis_iter_mut(Axis(0)).for_each(|_eqn| {
        Zip::from(this_equation);
    });
}
