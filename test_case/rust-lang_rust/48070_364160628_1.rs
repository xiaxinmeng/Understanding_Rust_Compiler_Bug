
% rustc issue-48070.rs -Z borrowck=mir -Z nll -Z two-phase-borrows
error[E0499]: cannot borrow `foo` as mutable more than once at a time
  --> issue-48070.rs:15:5
   |
15 | /     match 22 {
16 | |         22 => &mut foo,
17 | |         44 => foo.twiddle(),
   | |               --- first mutable borrow occurs here
18 | |         _ => foo.twaddle(),
19 | |     }.emit();
   | |_____^ second mutable borrow occurs here

error: aborting due to previous error

% 
