rs
// CHECK: { {{i8\*|ptr}}, {{i8\*|ptr}} } @dyn_star({{i8\*|ptr}} noundef align 1 %x.0, {{i8\*|ptr}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %x.1)
#[no_mangle]
pub fn dyn_star(x: dyn* Drop) -> dyn* Drop {
  x
}
