rust
trait MyTrait
    where Self::Assoc: Iterator<Item: Debug>,
{
    type Assoc;
}
