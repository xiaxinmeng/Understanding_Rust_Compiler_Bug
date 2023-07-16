rust
pub enum Equation<A, B> {
    Mul(A, B),
    Value(A),
}

impl<A, B> Default for Equation<A, B>
where
    A: Default,
{
    fn default() -> Self {
        Equation::Value(A::default())
    }
}

impl<A, B, T> From<T> for Equation<Equation<A, B>, Equation<A, B>>
where
    T: ToString,
    Equation<A, B>: From<T>,
    A: Default,
{
    fn from(input: T) -> Self {
        let left: Equation<A, B> = input.to_string().into();
        let right = Equation::default();
        let result: Equation<Equation<A, B>, Equation<A, B>> = Equation::Mul(left, right);
        return result;
    }
}

fn main() {
    println!("nothing to see here");
}
