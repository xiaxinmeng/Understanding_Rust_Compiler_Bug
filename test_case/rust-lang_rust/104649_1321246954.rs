rust
trait Project {
    type Assoc;
}

pub struct Wrap<T>(T);

impl<T> Project for Wrap<T> {
    type Assoc = Result<T, ()>;
}

pub trait Stream {
    type Item;

    fn get_projected<Fut, F>(self, f: F) -> Self::Item
    where
        F: FnMut(Self::Item) -> Fut,
        Self: Sized,
    {
        loop {}
    }
}

pub struct ProjectFnOutput<F>(F);

impl<F> Stream for ProjectFnOutput<F>
where
    F: GetFnOutput,
    F::Output: Project,
{
    type Item = <F::Output as Project>::Assoc;
}

pub trait GetFnOutput {
    type Output;
}

impl<T, R> GetFnOutput for T
where
    T: FnOnce() -> R,
{
    type Output = R;
}

fn main() {
    let proj = ProjectFnOutput(|| {
        Wrap(
            // This is `Result<(), _1>` but since no type annotations are provided the tpye of _1 is unknown.
            Ok(()),
        )
    });

    proj.get_projected(|b| async {
        // The type of `b` is `Result<Result<(), _1>, ()> where _1 is unknown.
        match b {
            Ok(Ok(url)) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
