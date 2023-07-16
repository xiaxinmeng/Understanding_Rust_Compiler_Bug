
test.rs:4:16: 4:44 error: associated type `anything_here_kills_it` not found for `Self` [E0220]
test.rs:4     type B = C<Self::anything_here_kills_it>;
                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
