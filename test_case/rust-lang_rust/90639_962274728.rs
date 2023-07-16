plain
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.0
    Checking filetime v0.2.14
    Checking dirs-next v2.0.0
error[E0599]: no method named `find_by_name_and_kind` found for reference `&[&AssocItem]` in the current scope
   |
   |
61 |             cx.tcx.associated_items(iter_did).find_by_name_and_kind(
   |                                               ^^^^^^^^^^^^^^^^^^^^^ method not found in `&[&AssocItem]`
    Checking rand_core v0.6.2
    Checking term v0.7.0
    Checking rand_core v0.5.1
For more information about this error, try `rustc --explain E0599`.
