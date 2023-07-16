
// CHECK-LABEL: reg_i32:
// CHECK: #APP
// CHECK: lgr r{{[0-15]+}}, r{{[0-15]+}}
// CHECK: #NO_APP
check!(reg_i32, i32, reg, "lgr");
