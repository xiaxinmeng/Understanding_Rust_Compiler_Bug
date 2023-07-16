plain
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.28
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0782]: trait objects must include the `dyn` keyword
     |
586  |  / macro_rules! event {
586  |  / macro_rules! event {
587  |  |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  |  |         $crate::__tracing_log!(
589  |  |             target: $target,
...     |
661  |  |                     &$crate::valueset!(meta.fields(), $($fields)*)
...     |
...     |
667  | /|         $crate::event!(
668  | ||             target: $target,
669  | ||             $lvl,
670  | ||             { message = format_args!($($arg)+), $($fields)* }
     | ||_________- in this macro invocation (#3)
...     |
791  |  |     );
792  |  | }
792  |  | }
     |  | -
     |  | |
     |  |_in this expansion of `$crate::event!` (#2)
     |    in this expansion of `$crate::event!` (#3)
1017 | /  macro_rules! debug {
1017 | /  macro_rules! debug {
1018 | |      (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 | |          $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
...    |
1186 | /          $crate::event!(
1186 | /          $crate::event!(
1187 |                target: module_path!(),
1188 |                $crate::Level::DEBUG,
1189 |                {},
1191 | |          )
     | |__________- in this macro invocation (#2)
1192 | |      );
1193 | |  }
1193 | |  }
     | |__- in this expansion of `debug!` (#1)
...
1925 |    macro_rules! valueset {
     |  __-
     | |
1926 | |
1926 | |
1927 | |      // === base case ===
1928 | |      (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
...    |
1955 | |              @ { $($out),*, (&$next, Some(&$val as &Value)) },
...    |
...    |
2070 |                $fields.value_set($crate::valueset!(
     |  ________________________________-
2071 |                    @ { },
2072 |                    iter.next().expect("FieldSet corrupted (this is a bug)"),
2073 |                    $($kvs)+
     | |______________- in this macro invocation (#5)
...
2081 | |      };
2082 | |  }
2082 | |  }
     | |  -
     | |__|
     | |__in this expansion of `$crate::valueset!` (#4)
     |    in this expansion of `$crate::valueset!` (#5)
    ::: compiler/rustc_data_structures/src/graph/implementation/mod.rs:149:9
     |
     |
149  |            debug!("graph: add_edge({:?}, {:?}, {:?})", source, target, data);
     |
     |
help: add `dyn` keyword before this trait
     |
1955 |             @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },


error[E0782]: trait objects must include the `dyn` keyword
     |
586  |   macro_rules! event {
     |  _-
     | |_|
     | |_|
     | |
587  | |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  | |         $crate::__tracing_log!(
589  | |             target: $target,
...    |
661  | |                     &$crate::valueset!(meta.fields(), $($fields)*)
...    |
667  | /         $crate::event!(
668  |               target: $target,
669  |               $lvl,
669  |               $lvl,
670  |               { message = format_args!($($arg)+), $($fields)* }
     | |_________- in this macro invocation (#3)
...
791  | |     );
792  | | }
792  | | }
     | | -
     | |_|
     | |_in this expansion of `$crate::event!` (#2)
     |   in this expansion of `$crate::event!` (#3)
1017 | / macro_rules! debug {
1017 | / macro_rules! debug {
1018 |       (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 |           $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
...
1186 | /         $crate::event!(
1186 | /         $crate::event!(
1187 | |             target: module_path!(),
1188 | |             $crate::Level::DEBUG,
1189 | |             {},
1191 | |         )
     | |_________- in this macro invocation (#2)
1192 |       );
1193 | | }
1193 | | }
     | |_- in this expansion of `debug!` (#1)
...
1925 |   macro_rules! valueset {
     | |_|
     | |
1926 | |
1926 | |
1927 | |     // === base case ===
1928 | |     (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
...    |
1955 | |             @ { $($out),*, (&$next, Some(&$val as &Value)) },
...    |
...    |
2070 |               $fields.value_set($crate::valueset!(
     |  _______________________________-
2071 |                   @ { },
2072 |                   iter.next().expect("FieldSet corrupted (this is a bug)"),
2073 |                   $($kvs)+
     | |_____________- in this macro invocation (#5)
...
2081 | |     };
2082 | | }
2082 | | }
     | | -
     | |_|
     | |_in this expansion of `$crate::valueset!` (#4)
     |   in this expansion of `$crate::valueset!` (#5)
    ::: compiler/rustc_data_structures/src/graph/scc/mod.rs:135:9
     |
135  | /         debug!(
135  | /         debug!(
136  | |             "create_scc({:?}) successors={:?}",
137  | |             self.ranges.len(),
138  | |             &self.all_successors[all_successors_start..all_successors_end],
     | |__________- in this macro invocation (#1)
     |
     |
help: add `dyn` keyword before this trait
     |
1955 |             @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },


error[E0782]: trait objects must include the `dyn` keyword
     |
586  |  / macro_rules! event {
586  |  / macro_rules! event {
587  |  |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  |  |         $crate::__tracing_log!(
589  |  |             target: $target,
...     |
661  |  |                     &$crate::valueset!(meta.fields(), $($fields)*)
...     |
...     |
667  | /|         $crate::event!(
668  | ||             target: $target,
669  | ||             $lvl,
670  | ||             { message = format_args!($($arg)+), $($fields)* }
     | ||_________- in this macro invocation (#3)
...     |
791  |  |     );
792  |  | }
792  |  | }
     |  | -
     |  | |
     |  |_in this expansion of `$crate::event!` (#2)
     |    in this expansion of `$crate::event!` (#3)
1017 | /  macro_rules! debug {
1017 | /  macro_rules! debug {
1018 | |      (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 | |          $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
...    |
1186 | /          $crate::event!(
1186 | /          $crate::event!(
1187 |                target: module_path!(),
1188 |                $crate::Level::DEBUG,
1189 |                {},
1191 | |          )
     | |__________- in this macro invocation (#2)
1192 | |      );
1193 | |  }
1193 | |  }
     | |__- in this expansion of `debug!` (#1)
...
1925 |    macro_rules! valueset {
     |  __-
     | |
1926 | |
1926 | |
1927 | |      // === base case ===
1928 | |      (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
...    |
1955 | |              @ { $($out),*, (&$next, Some(&$val as &Value)) },
...    |
...    |
2070 |                $fields.value_set($crate::valueset!(
     |  ________________________________-
2071 |                    @ { },
2072 |                    iter.next().expect("FieldSet corrupted (this is a bug)"),
2073 |                    $($kvs)+
     | |______________- in this macro invocation (#5)
...
2081 | |      };
2082 | |  }
2082 | |  }
     | |  -
     | |__|
     | |__in this expansion of `$crate::valueset!` (#4)
     |    in this expansion of `$crate::valueset!` (#5)
    ::: compiler/rustc_data_structures/src/graph/scc/mod.rs:305:13
     |
     |
305  |                debug!("find_state(r = {:?} in state {:?})", node, self.node_states[node]);
     |
     |
help: add `dyn` keyword before this trait
     |
1955 |             @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },


error[E0782]: trait objects must include the `dyn` keyword
     |
586  |  / macro_rules! event {
586  |  / macro_rules! event {
587  |  |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  |  |         $crate::__tracing_log!(
589  |  |             target: $target,
...     |
661  |  |                     &$crate::valueset!(meta.fields(), $($fields)*)
...     |
...     |
667  | /|         $crate::event!(
668  | ||             target: $target,
669  | ||             $lvl,
670  | ||             { message = format_args!($($arg)+), $($fields)* }
     | ||_________- in this macro invocation (#3)
...     |
791  |  |     );
792  |  | }
792  |  | }
     |  | -
     |  | |
     |  |_in this expansion of `$crate::event!` (#2)
     |    in this expansion of `$crate::event!` (#3)
1017 | /  macro_rules! debug {
1017 | /  macro_rules! debug {
1018 | |      (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 | |          $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
...    |
1186 | /          $crate::event!(
1186 | /          $crate::event!(
1187 |                target: module_path!(),
1188 |                $crate::Level::DEBUG,
1189 |                {},
1191 | |          )
     | |__________- in this macro invocation (#2)
1192 | |      );
1193 | |  }
1193 | |  }
     | |__- in this expansion of `debug!` (#1)
...
1925 |    macro_rules! valueset {
     |  __-
     | |
1926 | |
1926 | |
1927 | |      // === base case ===
1928 | |      (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
...    |
1955 | |              @ { $($out),*, (&$next, Some(&$val as &Value)) },
...    |
...    |
2070 |                $fields.value_set($crate::valueset!(
     |  ________________________________-
2071 |                    @ { },
2072 |                    iter.next().expect("FieldSet corrupted (this is a bug)"),
2073 |                    $($kvs)+
     | |______________- in this macro invocation (#5)
...
2081 | |      };
2082 | |  }
2082 | |  }
     | |  -
     | |__|
     | |__in this expansion of `$crate::valueset!` (#4)
     |    in this expansion of `$crate::valueset!` (#5)
    ::: compiler/rustc_data_structures/src/graph/scc/mod.rs:375:13
     |
     |
375  |                debug!("find_state: parent_state = {:?}", node_state);
     |
     |
help: add `dyn` keyword before this trait
     |
1955 |             @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },


error[E0782]: trait objects must include the `dyn` keyword
     |
586  |  / macro_rules! event {
586  |  / macro_rules! event {
587  |  |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  |  |         $crate::__tracing_log!(
589  |  |             target: $target,
...     |
661  |  |                     &$crate::valueset!(meta.fields(), $($fields)*)
...     |
...     |
667  | /|         $crate::event!(
668  | ||             target: $target,
669  | ||             $lvl,
670  | ||             { message = format_args!($($arg)+), $($fields)* }
     | ||_________- in this macro invocation (#3)
...     |
791  |  |     );
792  |  | }
792  |  | }
     |  | -
     |  | |
     |  |_in this expansion of `$crate::event!` (#2)
     |    in this expansion of `$crate::event!` (#3)
1017 | /  macro_rules! debug {
1017 | /  macro_rules! debug {
1018 | |      (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 | |          $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
...    |
1186 | /          $crate::event!(
1186 | /          $crate::event!(
1187 |                target: module_path!(),
1188 |                $crate::Level::DEBUG,
1189 |                {},
1191 | |          )
     | |__________- in this macro invocation (#2)
1192 | |      );
1193 | |  }
1193 | |  }
     | |__- in this expansion of `debug!` (#1)
...
1925 |    macro_rules! valueset {
     |  __-
     | |
1926 | |
1926 | |
1927 | |      // === base case ===
1928 | |      (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
...    |
1955 | |              @ { $($out),*, (&$next, Some(&$val as &Value)) },
...    |
...    |
2070 |                $fields.value_set($crate::valueset!(
     |  ________________________________-
2071 |                    @ { },
2072 |                    iter.next().expect("FieldSet corrupted (this is a bug)"),
2073 |                    $($kvs)+
     | |______________- in this macro invocation (#5)
...
2081 | |      };
2082 | |  }
2082 | |  }
     | |  -
     | |__|
     | |__in this expansion of `$crate::valueset!` (#4)
     |    in this expansion of `$crate::valueset!` (#5)
    ::: compiler/rustc_data_structures/src/obligation_forest/mod.rs:341:13
     |
     |
341  |                debug!("register_obligation_at: ignoring already done obligation: {:?}", obligation);
     |
     |
help: add `dyn` keyword before this trait
     |
1955 |             @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },


error[E0782]: trait objects must include the `dyn` keyword
     |
586  |   macro_rules! event {
     |  _-
     | |_|
     | |_|
     | |
587  | |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  | |         $crate::__tracing_log!(
589  | |             target: $target,
...    |
661  | |                     &$crate::valueset!(meta.fields(), $($fields)*)
...    |
667  | /         $crate::event!(
668  |               target: $target,
669  |               $lvl,
669  |               $lvl,
670  |               { message = format_args!($($arg)+), $($fields)* }
     | |_________- in this macro invocation (#3)
...
791  | |     );
792  | | }
792  | | }
     | | -
     | |_|
     | |_in this expansion of `$crate::event!` (#2)
     |   in this expansion of `$crate::event!` (#3)
1434 | / macro_rules! warn {
1434 | / macro_rules! warn {
1435 |        (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1436 |           $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
...
1603 | /         $crate::event!(
1603 | /         $crate::event!(
1604 | |             target: module_path!(),
1605 | |             $crate::Level::WARN,
1606 | |             {},
1608 | |         )
     | |_________- in this macro invocation (#2)
1609 |       );
1610 | | }
1610 | | }
     | |_- in this expansion of `warn!` (#1)
...
1925 |   macro_rules! valueset {
     | |_|
     | |
1926 | |
1926 | |
1927 | |     // === base case ===
1928 | |     (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
...    |
1955 | |             @ { $($out),*, (&$next, Some(&$val as &Value)) },
...    |
...    |
2070 |               $fields.value_set($crate::valueset!(
     |  _______________________________-
2071 |                   @ { },
2072 |                   iter.next().expect("FieldSet corrupted (this is a bug)"),
2073 |                   $($kvs)+
     | |_____________- in this macro invocation (#5)
...
2081 | |     };
2082 | | }
2082 | | }
     | | -
     | |_|
     | |_in this expansion of `$crate::valueset!` (#4)
     |   in this expansion of `$crate::valueset!` (#5)
    ::: compiler/rustc_data_structures/src/profiling.rs:470:17
     |
470  | /                 warn!(
470  | /                 warn!(
471  | |                     "Unknown self-profiler events specified: {}. Available options are: {}.",
472  | |                     unknown_events.join(", "),
473  | |                     EVENT_FILTERS_BY_NAME
...    |
477  | |                         .join(", ")
     | |__________________- in this macro invocation (#1)
     |
     |
help: add `dyn` keyword before this trait
     |
1955 |             @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },


error[E0782]: trait objects must include the `dyn` keyword
     |
586  | / macro_rules! event {
586  | / macro_rules! event {
587  | |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  | |         $crate::__tracing_log!(
589  | |             target: $target,
...    |
661  | |                     &$crate::valueset!(meta.fields(), $($fields)*)
...    |
791  | |     );
792  | | }
792  | | }
     | |_- in this expansion of `$crate::event!` (#2)
1017 | / macro_rules! debug {
1017 | / macro_rules! debug {
1018 | |     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 | |         $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
