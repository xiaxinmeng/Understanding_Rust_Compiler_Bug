
elevator.rs:12:12: 12:28 warning: the parameter type `T` may not live long enoug
h [E0309]
elevator.rs:12     result.push(lift(this));
                          ^~~~~~~~~~~~~~~~
elevator.rs:12:12: 12:28 help: run `rustc --explain E0309` to see a detailed explanation
elevator.rs:12:12: 12:28 help: consider adding an explicit lifetime bound `T: 'tcx`...
elevator.rs:12:12: 12:28 note: this warning results from recent bug fixes and clarifications; it will become a HARD ERROR in the next release. See RFC 1214 for details.
elevator.rs:12     result.push(lift(this));
                          ^~~~~~~~~~~~~~~~
elevator.rs:12:12: 12:28 note: ...so that the reference type `&collections::vec::Vec<<T as Lift<'_>>::Lifted>` does not outlive the data it points at
elevator.rs:12     result.push(lift(this));
                          ^~~~~~~~~~~~~~~~
