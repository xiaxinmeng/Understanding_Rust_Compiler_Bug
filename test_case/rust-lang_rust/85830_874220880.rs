plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |
15   | use rustc_span::def_id::LOCAL_CRATE;
     |

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:46
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
...    |
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:157:5
    |
157 |     is_private_dep => { cdata.private_dep }
    |     ^^^^^^^^^^^^^^ expected `()`, found fn item
    = note: expected unit type `()`
    = note: expected unit type `()`
                 found fn item `for<'tcx> fn(TyCtxt<'tcx>, CrateNum) -> bool {provide_extern::is_private_dep}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_metadata`
error: could not compile `rustc_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&(CrateNum, DefId)` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&(CrateNum, DefId)`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&DefId`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /     macro_rules! get_provider {
272  | /     macro_rules! get_provider {
273  | |         ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |             $tcx.queries.local_providers.$name
275  | |         }};
276  | |         ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |             let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                                 ^^^^^^^^^^^ method not found in `&CrateNum`
...    |
285  | |             get_provider!([$($modifiers)*][$($args)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
286  | |         };
286  | |         };
287  | |     }
     | |     -
     | |     |
     | |     in this expansion of `get_provider!` (#3)
     | |_____in this expansion of `get_provider!` (#4)
     |       in this expansion of `get_provider!` (#5)
289  | /     macro_rules! define_queries {
289  | /     macro_rules! define_queries {
290  | |         (<$tcx:tt>
291  | |          $($(#[$attr:meta])*
292  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                     get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |         }
466  | |     }
466  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&CrateNum` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&CrateNum`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error[E0599]: no method named `query_crate` found for reference `&DefId` in the current scope
    --> compiler/rustc_query_impl/src/plumbing.rs:277:29
272  | /   macro_rules! get_provider {
272  | /   macro_rules! get_provider {
273  | |       ([][$tcx:expr, $name:ident, $key:expr]) => {{
274  | |           $tcx.queries.local_providers.$name
275  | |       }};
276  | |       ([(separate_provide_extern) $($rest:tt)*][$tcx:expr, $name:ident, $key:expr]) => {{
277  | |           let is_local = $key.query_crate() == LOCAL_CRATE;
     | |                               ^^^^^^^^^^^ method not found in `&DefId`
286  | |       };
287  | |   }
287  | |   }
     | |___- in this expansion of `get_provider!` (#3)
288  | 
289  | /   macro_rules! define_queries {
290  | |       (<$tcx:tt>
291  | |        $($(#[$attr:meta])*
292  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
376  | |                   get_provider!([$($modifiers)*][tcx, $name, key])
...    |
465  | |       }
466  | |   }
466  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:50:1
     |
50   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
