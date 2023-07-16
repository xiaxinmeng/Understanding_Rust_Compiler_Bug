
trait Dc<Elem> where Elem: Add + AddAssign + Sub + SubAssign + Neg + Rand {
    type Sum = <Elem as Add>::Output;
[...]
}
