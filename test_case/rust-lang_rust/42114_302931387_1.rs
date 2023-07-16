rust
trait Prove<'a> : 'a {}

impl<'a> Prove<'a> for &'a () where Self: 'a {}

fn doesnt_work()
    where for<'a> &'a (): Prove<'a>
{}
