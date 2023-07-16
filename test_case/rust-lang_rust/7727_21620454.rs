 rust
enum SyntaxWrapper<T> {
    Plain(T),
    Quote(T),
    MacroInvocation(Mac)
}

type Expr = SyntaxWrapper<Exp_r>;
type Item = SyntaxWrapper<Item_>;
type Method = SyntaxWrapper<Method_>;
type Ty = SyntaxWrapper<Ty_>;
// ...
