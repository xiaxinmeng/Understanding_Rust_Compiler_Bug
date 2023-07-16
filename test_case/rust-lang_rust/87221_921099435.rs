rust
// `cfg_attr` is evaluated first, and written second so the lint is reported
// however, we cannot reorder the attributes because `my_trait_helper` is not in scope before `#[derive(MyTrait)]`
// but we can use `cfg_eval` to both evaluate `cfg_attr` and keep relative order of `MyTrait` and `my_trait_helper`
#[derive(MyTrait)]
#[cfg_attr(my_feature, my_trait_helper)]
struct S;
