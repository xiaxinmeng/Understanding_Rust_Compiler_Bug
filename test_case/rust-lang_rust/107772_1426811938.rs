
// CHECK: @trait_borrow({{\{\}\*|ptr}} noundef nonnull align 1 %_1.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %_1.1)
// FIXME #25759 This should also have `nocapture`
#[no_mangle]
pub fn trait_borrow(_: &dyn Drop) {
}

// CHECK: @option_trait_borrow({{i8\*|ptr}} noundef align 1 %x.0, {{i8\*|ptr}} %x.1)
#[no_mangle]
pub fn option_trait_borrow(x: Option<&dyn Drop>) {
}
