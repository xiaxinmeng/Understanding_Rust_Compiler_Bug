 rust
// note: haven't tried compiling this

trait TyFn<In> { type Out; }

enum Identity<A> {}
impl<A> TyFn<A> for Identity<A> { type Out = A; }

enum Dup<A> {}
impl<A> TyFn<A> for Dup<A> { type Out = (A, A); }

trait IsComposite { type Fst; type Snd; }
impl<A, B> IsComposite for (A, B) { type Fst = A; type Snd = B; }
enum Swap<A: IsComposite> {}
impl<A: IsComposite> TyFn<A> for Swap<A> { type Out = (<A as IsComposite>::Snd, <A as IsComposite>::Fst); }

struct Helper<X, TF: TyFn<X>>;
impl<A, TF: TyFn<A>> Fn(A) for Helper<A, TF> {
    type R = <TF as TyFn<A>>::Out;
    …
}
