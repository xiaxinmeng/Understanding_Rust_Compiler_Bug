 rust
pub trait ProxiedBy<T> {}
impl<T> ProxiedBy<T> for T {}

// New subtrait:
pub trait ProxiedBySubtrait<T>: ProxiedBy<T> {}
impl<T> ProxiedBySubtrait<T> for T {}

pub trait SomeConstraint {}

struct Performer<OnT>(OnT);

impl<OnT> Performer<OnT> {
    fn perform<QueryT, ProxyT>(&self, query: QueryT)
        where QueryT: ProxiedBySubtrait<ProxyT>,  // Was QueryT: ProxiedBy<ProxyT>
              OnT: ProxiedBy<ProxyT>,
              ProxyT: SomeConstraint {}
}

fn good_again<OnT, IrrelevantT>(performer: Performer<OnT>, on: OnT)
        where OnT: SomeConstraint + ProxiedBy<IrrelevantT>,
              OnT: ProxiedBy<OnT>  // This constraint should be unnecessary!
{
    performer.perform(on);
}
