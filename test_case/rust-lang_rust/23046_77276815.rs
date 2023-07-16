 Rust
pub enum Expr<'var, VAR> {
    Let(Box<Expr<'var, VAR>>,
        Box<for<'v: 'var> Fn(Expr<'v, VAR>) -> Expr<'v, VAR> + 'var>)
}

pub fn add<'var, VAR>
                      (a: Expr<'var, VAR>, b: Expr<'var, VAR>) -> Expr<'var, VAR> {
    loop {}
}

pub fn let_<'var, VAR, F: for<'v: 'var> Fn(Expr<'v, VAR>) -> Expr<'v, VAR>>
                       (a: Expr<'var, VAR>, b: F) -> Expr<'var, VAR> {
    loop {}
}

fn main() {
    let ex =  (|x| {
        let_(add(x,x), |y| {
            let_(add(x, x), |x|x)})});
}
