rust
trait Ty<'a> {
    type V;
}

trait SIter: for<'a> Ty<'a> {
    fn f<F>(&self, f: F)
    where
        F: for<'r> Fn(<Self as Ty<'r>>::V);
}

struct S<I>(I);

impl<'a, I: Ty<'a>> Ty<'a> for S<I> {
    type V = <I as Ty<'a>>::V;
}

impl<I: SIter, Item> SIter for S<I>
where
    for<'r> S<I>: Ty<'r, V = Item>,
    for<'r> I: Ty<'r, V = Item>,
{
    fn f<F>(&self, f: F)
    where
        F: Fn(<Self as Ty>::V),
    {
        self.0.f(|item| help(&f, item))
    }
}

fn help<'a, T: Ty<'a>, F>(f: &F, t: <T as Ty<'a>>::V)
where
    F: Fn(<T as Ty>::V),
{
    f(t)
}

struct AA;
impl<'a> Ty<'a> for AA {
    type V = ();
}
impl SIter for AA {
    fn f<F>(&self, f: F)
    where
        F: for<'r> Fn(<Self as Ty<'r>>::V),
    {
        help(&f, ())
    }
}

fn main() {
    S(AA).f(|_| {})
}

