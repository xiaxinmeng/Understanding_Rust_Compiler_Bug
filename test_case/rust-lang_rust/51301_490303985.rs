
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:39:11
   |
39 |             .find(|(&event_type, _)|event == event_type)
   |                    ^^----------^^^^
   |                    | |
   |                    | data moved here
   |                    cannot move out of borrowed content
   |
note: move occurs because `event_type` has type `EventType`, which does not implement the `Copy` trait
  --> src/lib.rs:39:13
   |
39 |             .find(|(&event_type, _)|event == event_type)
   |                      ^^^^^^^^^^
