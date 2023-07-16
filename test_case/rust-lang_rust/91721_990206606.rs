plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  | /          join_internal! {
81  |                current_position: [
81  |                current_position: [
82  |                    $($underscores)*
...
91  |                ]
92  | |          }
    | |__________- in this macro invocation (#3)
    | |__________- in this macro invocation (#3)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
...   |
116 | |                          futures.as_mut().map_unchecked_mut(|it| {
    | |                          ^^^^^^^^^^^^^^^^ cannot borrow as mutable
145 | |      ),
146 | |  }
    | |  -
    | |__|
    | |__|
    | |__in this expansion of `join_internal!` (#2)
    |    in this expansion of `join_internal!` (#3)
   ::: library/core/tests/future.rs:35:17
    |
    |
35  |            let x = join!(async { 0 }).await;


error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  | /          join_internal! {
81  |                current_position: [
81  |                current_position: [
82  |                    $($underscores)*
...
91  |                ]
92  | |          }
    | |__________- in this macro invocation (#3)
    | |__________- in this macro invocation (#3)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
133 | |                      futures.as_mut().get_unchecked_mut()
    | |                      ^^^^^^^^^^^^^^^^ cannot borrow as mutable
...   |
145 | |      ),
145 | |      ),
146 | |  }
    | |  -
    | |__|
    | |__in this expansion of `join_internal!` (#2)
    |    in this expansion of `join_internal!` (#3)
   ::: library/core/tests/future.rs:35:17
    |
    |
35  |            let x = join!(async { 0 }).await;


error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |__|
    | |__|
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  |            join_internal! {
    |  __________-
    | |__________|
    | |
    | |
81  | |              current_position: [
82  | |                  $($underscores)*
...   |
91  | |              ]
92  | |          }
    | |          -
    | |          -
    | |__________|
    | |__________in this macro invocation (#3)
    |            in this macro invocation (#4)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
...   |
116 | |                          futures.as_mut().map_unchecked_mut(|it| {
    | |                          ^^^^^^^^^^^^^^^^ cannot borrow as mutable
145 | |      ),
146 | |  }
    | |  -
    | |__|
    | |__|
    | |__in this expansion of `join_internal!` (#2)
    | |__in this expansion of `join_internal!` (#3)
    |    in this expansion of `join_internal!` (#4)
   ::: library/core/tests/future.rs:38:17
    |
    |
38  |            let x = join!(async { 0 }, async { 1 }).await;


error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |__|
    | |__|
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  |            join_internal! {
    |  __________-
    | |__________|
    | |
    | |
81  | |              current_position: [
82  | |                  $($underscores)*
...   |
91  | |              ]
92  | |          }
    | |          -
    | |          -
    | |__________|
    | |__________in this macro invocation (#3)
    |            in this macro invocation (#4)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
133 | |                      futures.as_mut().get_unchecked_mut()
    | |                      ^^^^^^^^^^^^^^^^ cannot borrow as mutable
...   |
145 | |      ),
145 | |      ),
146 | |  }
    | |  -
    | |__|
    | |__in this expansion of `join_internal!` (#2)
    | |__in this expansion of `join_internal!` (#3)
    |    in this expansion of `join_internal!` (#4)
   ::: library/core/tests/future.rs:38:17
    |
    |
38  |            let x = join!(async { 0 }, async { 1 }).await;


error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |__|
    | |__|
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  |            join_internal! {
    |  __________-
    | |__________|
    | |
    | |
81  | |              current_position: [
82  | |                  $($underscores)*
...   |
91  | |              ]
92  | |          }
    | |          -
    | |          -
    | |__________|
    | |__________in this macro invocation (#3)
    |            in this macro invocation (#4)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
...   |
116 | |                          futures.as_mut().map_unchecked_mut(|it| {
    | |                          ^^^^^^^^^^^^^^^^ cannot borrow as mutable
145 | |      ),
146 | |  }
    | |  -
    | |__|
    | |__|
    | |__in this expansion of `join_internal!` (#2)
    | |__in this expansion of `join_internal!` (#3)
    |    in this expansion of `join_internal!` (#4)
   ::: library/core/tests/future.rs:41:17
    |
    |
41  |            let x = join!(async { 0 }, async { 1 }, async { 2 }).await;


error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |__|
    | |__|
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  |            join_internal! {
    |  __________-
    | |__________|
    | |
    | |
81  | |              current_position: [
82  | |                  $($underscores)*
...   |
91  | |              ]
92  | |          }
    | |          -
    | |          -
    | |__________|
    | |__________in this macro invocation (#3)
    |            in this macro invocation (#4)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
133 | |                      futures.as_mut().get_unchecked_mut()
    | |                      ^^^^^^^^^^^^^^^^ cannot borrow as mutable
...   |
145 | |      ),
145 | |      ),
146 | |  }
    | |  -
    | |__|
    | |__in this expansion of `join_internal!` (#2)
    | |__in this expansion of `join_internal!` (#3)
    |    in this expansion of `join_internal!` (#4)
   ::: library/core/tests/future.rs:41:17
    |
    |
41  |            let x = join!(async { 0 }, async { 1 }, async { 2 }).await;


error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  /  pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |      // Funnel through an internal macro not to leak implementation details.
50  |  |      join_internal! {
51  | ||          current_position: []
52  | ||          futures_and_positions: []
52  | ||          futures_and_positions: []
53  | ||          munching: [ $($fut)+ ]
    | ||______- in this macro invocation (#2)
55  |  |  }
55  |  |  }
    |  |__- in this expansion of `join!` (#1)
63  |     macro join_internal {
    |  ___-
    | |___|
    | |___|
    | |___|
    | |
64  | |       // Recursion step: map each future with its "position" (underscore count).
65  | |       (
66  | |           // Accumulate a token for each future that has been expanded: "_ _ _".
80  |             join_internal! {
    |  ___________-
    | |___________|
    | |
    | |
81  | |               current_position: [
82  | |                   $($underscores)*
...   |
91  | |               ]
92  | |           }
    | |           -
    | |           -
    | |___________|
    | |___________in this macro invocation (#3)
    |             in this macro invocation (#4)
...
109 | |               let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                   ------- help: consider changing this to be mutable: `mut futures`
...   |
116 | |                           futures.as_mut().map_unchecked_mut(|it| {
    | |                           ^^^^^^^^^^^^^^^^ cannot borrow as mutable
145 | |       ),
146 | |   }
    | |   -
    | |___|
    | |___|
    | |___in this expansion of `join_internal!` (#2)
    | |___in this expansion of `join_internal!` (#3)
    |     in this expansion of `join_internal!` (#4)
   ::: library/core/tests/future.rs:44:17
    |
    |
44  |             let x = join!(
45  |   |             poll_n(0, 1),
46  |   |             poll_n(1, 5),
47  |   |             poll_n(2, 2),
...     |
...     |
52  |   |             poll_n(7, 1)
53  |   |         )
    |   |_________- in this macro invocation (#1)

error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  /  pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |      // Funnel through an internal macro not to leak implementation details.
50  |  |      join_internal! {
51  | ||          current_position: []
52  | ||          futures_and_positions: []
52  | ||          futures_and_positions: []
53  | ||          munching: [ $($fut)+ ]
    | ||______- in this macro invocation (#2)
55  |  |  }
55  |  |  }
    |  |__- in this expansion of `join!` (#1)
63  |     macro join_internal {
    |  ___-
    | |___|
    | |___|
    | |___|
    | |
64  | |       // Recursion step: map each future with its "position" (underscore count).
65  | |       (
66  | |           // Accumulate a token for each future that has been expanded: "_ _ _".
80  |             join_internal! {
    |  ___________-
    | |___________|
    | |
    | |
81  | |               current_position: [
82  | |                   $($underscores)*
...   |
91  | |               ]
92  | |           }
    | |           -
    | |           -
    | |___________|
    | |___________in this macro invocation (#3)
    |             in this macro invocation (#4)
...
109 | |               let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                   ------- help: consider changing this to be mutable: `mut futures`
133 | |                       futures.as_mut().get_unchecked_mut()
    | |                       ^^^^^^^^^^^^^^^^ cannot borrow as mutable
...   |
145 | |       ),
145 | |       ),
146 | |   }
    | |   -
    | |___|
    | |___in this expansion of `join_internal!` (#2)
    | |___in this expansion of `join_internal!` (#3)
    |     in this expansion of `join_internal!` (#4)
   ::: library/core/tests/future.rs:44:17
    |
    |
44  |             let x = join!(
45  |   |             poll_n(0, 1),
46  |   |             poll_n(1, 5),
47  |   |             poll_n(2, 2),
...     |
...     |
52  |   |             poll_n(7, 1)
53  |   |         )
    |   |_________- in this macro invocation (#1)

error[E0596]: cannot borrow `futures` as mutable, as it is not declared as mutable
    |
    |
48  |  / pub macro join( $($fut:expr),+ $(,)? ) {
49  |  |     // Funnel through an internal macro not to leak implementation details.
50  |  |     join_internal! {
51  | ||         current_position: []
52  | ||         futures_and_positions: []
52  | ||         futures_and_positions: []
53  | ||         munching: [ $($fut)+ ]
    | ||_____- in this macro invocation (#2)
55  |  | }
55  |  | }
    |  |_- in this expansion of `join!` (#1)
63  |    macro join_internal {
    |  __-
    | |__|
    | |
    | |
64  | |      // Recursion step: map each future with its "position" (underscore count).
65  | |      (
66  | |          // Accumulate a token for each future that has been expanded: "_ _ _".
80  | /          join_internal! {
81  |                current_position: [
81  |                current_position: [
82  |                    $($underscores)*
...
91  |                ]
92  | |          }
    | |__________- in this macro invocation (#3)
    | |__________- in this macro invocation (#3)
...
109 | |              let futures = unsafe { Pin::new_unchecked(&mut futures) };
    | |                  ------- help: consider changing this to be mutable: `mut futures`
