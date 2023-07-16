 rust
pub fn problem_creator<'a, P>(_: &'a P) 
    where &'a P: IntoIterator<Item = &'a f64> 
{}

pub fn ok() {
    let points = Vec::new();
    problem_creator(&points)
}

pub fn ok2() {
    let points: Vec<f64> = Vec::new();
    problem_creator(&points)
}

pub fn ice<'a, Q>(_: &'a Q)
    where &'a Q: IntoIterator<Item = &'a f64>
{
    let points = Vec::new();
    problem_creator(&points)
}

pub fn broken_typechecking<'a, Q>(_: &'a Q)
    where &'a Q: IntoIterator<Item = &'a f64>
{
    let points: Vec<f64> = Vec::new();
    problem_creator(&points)
}

fn main() {}
