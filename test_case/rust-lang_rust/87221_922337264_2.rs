rust
#[allow(out_of_order_expansion)]
#[derive(MyTrait)]
#[cfg_attr(my_feature, my_trait_helper)]
struct S;
