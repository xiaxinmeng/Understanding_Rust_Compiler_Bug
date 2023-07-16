plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `bytes` in this scope
    --> src/librustdoc/html/render/print_item.rs:1537:17
     |
1537 |                 bytes = ty.layout.size.bytes();

error[E0425]: cannot find value `bytes` in this scope
    --> src/librustdoc/html/render/print_item.rs:1541:28
     |
     |
1541 |                     size = bytes,
     |                            ^^^^^ not found in this scope

error[E0425]: cannot find value `bytes` in this scope
    --> src/librustdoc/html/render/print_item.rs:1542:29
     |
1542 |                     pl = if bytes == 1 { "" } else { "s" },


error[E0609]: no field `layout` on type `&TyS<'_>`
    --> src/librustdoc/html/render/print_item.rs:1537:28
     |
1537 |                 bytes = ty.layout.size.bytes();

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0425, E0609.
