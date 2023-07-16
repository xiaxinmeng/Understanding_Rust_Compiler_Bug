rust
let arbitrary = Arbitrary {
    _t: core::marker::PhantomData::<dyn ArbitraryTrait>
};

arbitrary.method();
