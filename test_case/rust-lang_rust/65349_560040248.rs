
error[E0308]: mismatched types
  --> $DIR/raw-ptr-const-param.rs:7:38
   |
LL |     let _: Const<{15 as *const _}> = Const::<{10 as *const _}>;
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{pointer#4}`, found `{pointer#89}`
   |
   = note: expected type `Const<{pointer#4}>`
              found type `Const<{pointer#89}>`
