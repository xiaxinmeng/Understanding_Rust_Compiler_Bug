rust
trait MyTrait
    where Self::Assoc: Iterator,
        <Self::Assoc as Iterator>::Item: Debug,
{
    type Assoc;
}
