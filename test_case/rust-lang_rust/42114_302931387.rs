rust
trait Prove<'a> : 'a {}

impl<'a> Prove<'a> for &'a () {}

fn does_work()
    where for<'a> &'a (): Prove<'a>
{}
