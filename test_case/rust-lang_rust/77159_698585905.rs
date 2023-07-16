
trait Trait<'lt>: 'lt + Sized + Op<&'lt Self> + Op<Self> {}
