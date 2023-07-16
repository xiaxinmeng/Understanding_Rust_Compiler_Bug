
the trait `Sync` is not implemented for `dyn std::fmt::Display`

future is not `Send` as this value is used across an await

         $crate::fmt::Display::to_string(&$arg);
                                         ----- has type `&dyn std::fmt::Display` which is not `Send`
