
trait NegFoo : MarkerTrait { }
impl NegFoo for .. { }
impl<T:!Foo> !NegFoo for T { }
