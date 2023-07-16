
% rustc +1.20.0 issue-55625.rs
error[E0320]: overflow while adding drop-check rules for A<T>
 --> issue-55625.rs:1:10
  |
1 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for A<T>
 --> issue-55625.rs:1:10
  |
1 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for A<T>
 --> issue-55625.rs:1:10
  |
1 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
error[E0320]: overflow while adding drop-check rules for B<T>
 --> issue-55625.rs:3:8
  |
3 |     A1(B<T>),
  |        ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for A<T>
 --> issue-55625.rs:1:10
  |
1 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for B<std::option::Option<T>>
 --> issue-55625.rs:4:8
  |
4 |     A2(B<Option<T>>),
  |        ^^^^^^^^^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
rror[E0320]: overflow while adding drop-check rules for B<T>
 --> issue-55625.rs:7:10
  |
7 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for B<T>
 --> issue-55625.rs:7:10
  |
7 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for B<T>
 --> issue-55625.rs:7:10
  |
7 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for B<T>
 --> issue-55625.rs:7:10
  |
7 | #[derive(Clone)]
  |          ^^^^^
  |
  = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::o\
ption::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Optio\
n<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opt\
ion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<\
B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::optio\
n::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<\
std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option:\
:Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for std::boxed::Box<A<B<T>>>
  --> issue-55625.rs:10:8
   |
10 |     B2(Box<A<B<T>>>),
   |        ^^^^^^^^^^^^^
   |
   = note: overflowed on A<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::\
option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Opti\
on<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::op\
tion::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option\
<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::opti\
on::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B\
<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option\
::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<std::option::Option<B<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error: aborting due to 11 previous errors
